//! 量化模块
//!
//! 支持多种量化方法：
//! - INT8 对称量化
//! - INT4 非对称量化
//! - GPTQ
//! - AWQ

use ndarray::{Array2, Array3};
use std::collections::HashMap;

/// 量化类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantizationType {
    None,
    F32,
    F16,
    BF16,
    Int8,
    Int4,
    Int3,
    FP8,
    Q8_0,
    Q4_0,
    Q4_1,
}

/// 量化配置
#[derive(Debug, Clone)]
pub struct QuantizationConfig {
    /// 量化类型
    pub qtype: QuantizationType,
    /// 量化组大小
    pub group_size: usize,
    /// 是否使用混合量化
    pub mixed_quantization: bool,
}

impl Default for QuantizationConfig {
    fn default() -> Self {
        Self {
            qtype: QuantizationType::Int8,
            group_size: 128,
            mixed_quantization: true,
        }
    }
}

/// 量化后的权重
pub struct QuantizedWeight {
    /// 量化后的数据
    pub data: Vec<i8>,
    /// 缩放因子
    pub scales: Array2<f32>,
    /// 零偏移（用于非对称量化）
    pub zero_point: Option<Array2<f32>>,
    /// 原始形状
    pub original_shape: (usize, usize),
}

impl QuantizedWeight {
    /// 解量化
    pub fn dequantize(&self) -> Array2<f32> {
        let (rows, cols) = self.original_shape;
        let mut output = Array2::zeros((rows, cols));

        if let Some(ref zp) = self.zero_point {
            // 非对称量化: (q - zp) * scales
            for i in 0..rows {
                for j in 0..cols {
                    let q = self.data[i * cols + j] as f32;
                    output[[i, j]] = (q - zp
                        [[i / self.scales.shape()[1], j % self.scales.shape()[1]]])
                        * self.scales[[i / self.scales.shape()[1], j % self.scales.shape()[1]]];
                }
            }
        } else {
            // 对称量化: q * scales
            for i in 0..rows {
                for j in 0..cols {
                    let q = self.data[i * cols + j] as f32;
                    output[[i, j]] =
                        q * self.scales[[i / self.scales.shape()[1], j % self.scales.shape()[1]]];
                }
            }
        }

        output
    }
}

/// 量化器
pub struct Quantizer {
    config: QuantizationConfig,
}

impl Quantizer {
    pub fn new(config: QuantizationConfig) -> Self {
        Self { config }
    }

    /// 量化权重（INT8 对称）
    pub fn quantize_int8_symmetric(&self, weight: &Array2<f32>) -> QuantizedWeight {
        let shape = weight.shape();
        let rows = shape[0];
        let cols = shape[1];

        let mut data = Vec::with_capacity(rows * cols);
        let mut scales = Array2::zeros((rows, 1));

        for i in 0..rows {
            // 计算每行的最大绝对值
            let mut max_val = 0.0f32;
            for j in 0..cols {
                max_val = max_val.max(weight[[i, j]].abs());
            }

            // 计算缩放因子
            let scale = if max_val > 0.0 { 127.0 / max_val } else { 1.0 };

            scales[[i, 0]] = 1.0 / scale;

            // 量化
            for j in 0..cols {
                let q = (weight[[i, j]] * scale).round() as i8;
                data.push(q.clamp(-127, 127));
            }
        }

        QuantizedWeight {
            data,
            scales,
            zero_point: None,
            original_shape: (rows, cols),
        }
    }

    /// 量化权重（INT4 非对称）
    pub fn quantize_int4_asymmetric(&self, weight: &Array2<f32>) -> QuantizedWeight {
        let shape = weight.shape();
        let rows = shape[0];
        let cols = shape[1];

        let group_size = self.config.group_size;
        let num_groups = (cols + group_size - 1) / group_size;

        let mut data = Vec::new();
        let mut scales = Array2::zeros((rows, num_groups));
        let mut zero_points = Array2::zeros((rows, num_groups));

        for i in 0..rows {
            for g in 0..num_groups {
                let start = g * group_size;
                let end = (start + group_size).min(cols);

                // 找到 min 和 max
                let mut min_val = f32::MAX;
                let mut max_val = f32::MIN;
                for j in start..end {
                    min_val = min_val.min(weight[[i, j]]);
                    max_val = max_val.max(weight[[i, j]]);
                }

                // 计算量化和缩放
                let range = max_val - min_val;
                if range > 0.0 {
                    let scale = 15.0 / range;
                    let zp = -min_val * scale;

                    scales[[i, g]] = range / 15.0;
                    zero_points[[i, g]] = zp;

                    for j in start..end {
                        let q = ((weight[[i, j]] * scale + zp).round() as i8).clamp(0, 15);
                        data.push(q);
                    }
                } else {
                    scales[[i, g]] = 1.0;
                    zero_points[[i, g]] = 0.0;

                    for _ in start..end {
                        data.push(0i8);
                    }
                }
            }
        }

        // 压缩为半字节
        let compressed = Self::compress_int4_to_half_byte(&data);

        QuantizedWeight {
            data: compressed,
            scales,
            zero_point: Some(zero_points),
            original_shape: (rows, cols),
        }
    }

    /// 压缩 INT4 数据
    fn compress_int4_to_half_byte(data: &[i8]) -> Vec<i8> {
        let mut result = Vec::with_capacity(data.len() / 2);

        for chunk in data.chunks(2) {
            let mut byte = 0u8;
            if let Some(&v) = chunk.get(0) {
                byte = (v & 0x0F) as u8;
            }
            if let Some(&v) = chunk.get(1) {
                byte |= ((v & 0x0F) as u8) << 4;
            }
            result.push(byte as i8);
        }

        result
    }

    /// 动态量化（按通道）
    pub fn dynamic_quantize(&self, weight: &Array2<f32>) -> QuantizedWeight {
        let shape = weight.shape();
        let cols = shape[1];

        // 按列计算缩放
        let mut scales = Array2::zeros((1, cols));
        for j in 0..cols {
            let mut max_val = 0.0f32;
            for i in 0..shape[0] {
                max_val = max_val.max(weight[[i, j]].abs());
            }
            scales[[0, j]] = if max_val > 0.0 { 127.0 / max_val } else { 1.0 };
        }

        // 量化
        let mut data = Vec::with_capacity(weight.len());
        for i in 0..shape[0] {
            for j in 0..cols {
                let q = (weight[[i, j]] * scales[[0, j]]).round() as i8;
                data.push(q.clamp(-127, 127));
            }
        }

        QuantizedWeight {
            data,
            scales: scales.broadcast((shape[0], cols)).unwrap().to_owned(),
            zero_point: None,
            original_shape: (shape[0], shape[1]),
        }
    }
}

/// 反量化器
pub struct Dequantizer;

impl Dequantizer {
    /// 从 GGML 格式加载并反量化
    pub fn from_ggml(data: &[u8], shape: (usize, usize), qtype: QuantizationType) -> Array2<f32> {
        match qtype {
            QuantizationType::Int8 => Self::dequantize_int8(data, shape),
            QuantizationType::Int4 => Self::dequantize_int4(data, shape),
            _ => panic!("Unsupported quantization type"),
        }
    }

    fn dequantize_int8(data: &[u8], shape: (usize, usize)) -> Array2<f32> {
        let (rows, cols) = shape;
        let mut output = Array2::zeros((rows, cols));

        for i in 0..rows {
            for j in 0..cols {
                output[[i, j]] = data[i * cols + j] as f32;
            }
        }

        output
    }

    fn dequantize_int4(data: &[u8], shape: (usize, usize)) -> Array2<f32> {
        // 简化实现
        let (rows, cols) = shape;
        let mut output = Array2::zeros((rows, cols));

        for i in 0..rows {
            for j in 0..cols {
                let byte = data[i * (cols / 2) + j / 2];
                let q = if j % 2 == 0 {
                    (byte & 0x0F) as i8
                } else {
                    ((byte >> 4) & 0x0F) as i8
                };
                output[[i, j]] = q as f32;
            }
        }

        output
    }
}
