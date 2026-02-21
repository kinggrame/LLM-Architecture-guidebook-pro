//! 模型加载器模块
//!
//! 支持从多种格式加载模型：
//! - GGML 格式（llama.cpp 使用）
//! - HuggingFace SafeTensors 格式
//! - PyTorch pickle 格式
//!
//! ## GGML 格式
//! GGML 是 llama.cpp 使用的二进制格式，特点：
//! - 内存映射支持（mmap）
//! - 量化支持（Q4_0, Q4_1, Q5_0, Q5_1, Q8_0）
//! - 无需 PyTorch 依赖
//!
//! ## SafeTensors 格式
//! HuggingFace 推荐的安全格式：
//! - Rust 实现：safetensors-rs
//! - 支持内存映射
//! - 支持 tensor 切片

use crate::model::{LayerWeights, ModelConfig, ModelType, ModelWeights, RoPE};
use crate::quantization::QuantizationType;
use crate::{LLMError, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// 加载结果
pub struct LoadResult {
    /// 模型配置
    pub config: ModelConfig,
    /// 模型权重
    pub weights: ModelWeights,
    /// 量化类型
    pub quantization: QuantizationType,
    /// RoPE 频率（如果使用）
    pub rope: Option<RoPE>,
    /// 模型元数据
    pub metadata: HashMap<String, String>,
}

/// 模型加载器
pub struct ModelLoader {
    /// 模型路径
    path: String,
    /// 模型类型
    #[allow(dead_code)]
    model_type: ModelType,
    /// 量化类型
    quantization: QuantizationType,
}

impl ModelLoader {
    /// 创建新的加载器
    pub fn new(path: impl Into<String>, model_type: ModelType) -> Self {
        Self {
            path: path.into(),
            model_type,
            quantization: QuantizationType::F16,
        }
    }

    /// 设置量化类型
    pub fn with_quantization(mut self, quantization: QuantizationType) -> Self {
        self.quantization = quantization;
        self
    }

    /// 加载模型
    pub fn load(&self) -> Result<LoadResult> {
        let path = Path::new(&self.path);

        if !path.exists() {
            return Err(LLMError::LoadError(format!(
                "Model file not found: {}",
                self.path
            )));
        }

        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        match extension {
            "gguf" | "ggml" | "bin" => self.load_ggml_format(),
            "safetensors" => self.load_safetensors(),
            "pt" | "pth" => self.load_pytorch(),
            _ => self.detect_and_load(),
        }
    }

    /// 自动检测格式并加载
    fn detect_and_load(&self) -> Result<LoadResult> {
        let mut file = File::open(&self.path)?;
        let mut header = [0u8; 12];

        if file
            .read(&mut header)
            .map_err(|e| LLMError::LoadError(e.to_string()))?
            < 12
        {
            return Err(LLMError::LoadError("File too small".to_string()));
        }

        // 检测 GGML/GGUF 魔数
        if &header[0..4] == b"ggml" || &header[0..4] == b"gguf" {
            return self.load_ggml_format();
        }

        // 尝试作为 safetensors 加载
        self.load_safetensors()
    }

    /// 从 GGML/GGUF 格式加载
    fn load_ggml_format(&self) -> Result<LoadResult> {
        let file = File::open(&self.path)?;
        let mut reader = BufReader::new(file);

        // 读取魔数
        let mut magic = [0u8; 4];
        reader
            .read_exact(&mut magic)
            .map_err(|e| LLMError::LoadError(e.to_string()))?;

        let is_gguf = &magic == b"gguf";

        // 读取模型类型
        let model_type = self.read_ggml_string(&mut reader)?;
        let model_type = match model_type.as_str() {
            "llama" => ModelType::LLaMA,
            "gpt2" => ModelType::GPT2,
            "gptj" => ModelType::GPTJ,
            "gpt-neoxx" => ModelType::GPTNeoX,
            _ => return Err(LLMError::UnsupportedModel(model_type)),
        };

        // 读取配置
        let mut params = HashMap::new();

        if is_gguf {
            // GGUF 格式：读取 kv tensors
            self.load_gguf_kv(&mut reader, &mut params)?;
        }

        let config = ModelConfig::from_params(model_type, &params);

        // 读取权重
        let weights = self.load_ggml_weights(&mut reader, &config)?;

        // 创建 RoPE（如果需要）
        let rope = config
            .rope_dim
            .map(|dim| RoPE::new(dim, config.max_seq_len, 10000.0));

        Ok(LoadResult {
            config,
            weights,
            quantization: self.quantization,
            rope,
            metadata: HashMap::new(),
        })
    }

    /// 读取 GGML 字符串
    fn read_ggml_string<R: Read>(&self, reader: &mut R) -> Result<String> {
        let len = match u64::from_le_bytes([0; 8]) {
            _ => {
                let mut len_buf = [0u8; 4];
                reader.read_exact(&mut len_buf)?;
                u32::from_le_bytes(len_buf) as usize
            }
        };

        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|e| LLMError::LoadError(e.to_string()))
    }

    /// 加载 GGUF KV tensors
    fn load_gguf_kv<R: Read>(
        &self,
        _reader: &mut R,
        _params: &mut HashMap<String, usize>,
    ) -> Result<()> {
        // GGUF 格式的 KV tensor 读取
        // 这里简化处理，实际需要完整的 GGUF 规范实现
        Ok(())
    }

    /// 加载 GGML 权重
    fn load_ggml_weights<R: Read>(
        &self,
        reader: &mut R,
        config: &ModelConfig,
    ) -> Result<ModelWeights> {
        // Token 嵌入
        let token_embedding =
            self.read_tensor_2d(reader, config.vocab_size, config.embedding_dim)?;

        // 层级权重
        let mut layers = Vec::with_capacity(config.num_layers);

        for _ in 0..config.num_layers {
            let layer = self.load_layer_weights(reader, config)?;
            layers.push(layer);
        }

        // 最终 Layer Norm
        let final_layernorm_gamma = self.read_tensor_2d(reader, 1, config.embedding_dim)?;
        let final_layernorm_beta = self.read_tensor_2d(reader, 1, config.embedding_dim)?;

        // LM Head（可能与 token_embedding 共享）
        let lm_head = self
            .read_tensor_2d(reader, config.vocab_size, config.embedding_dim)
            .ok();

        Ok(ModelWeights {
            token_embedding,
            layers,
            final_layernorm_gamma,
            final_layernorm_beta,
            lm_head,
        })
    }

    /// 加载单层权重
    fn load_layer_weights<R: Read>(
        &self,
        reader: &mut R,
        config: &ModelConfig,
    ) -> Result<LayerWeights> {
        let head_dim = config.head_dim;
        let hidden_dim = config.embedding_dim;
        let _num_heads = config.num_heads;
        let num_kv_heads = config.num_kv_heads;

        // 读取 Q、K、V 投影
        let wq = self.read_tensor_2d(reader, hidden_dim, hidden_dim)?;
        let wk = self.read_tensor_2d(reader, hidden_dim, num_kv_heads * head_dim)?;
        let wv = self.read_tensor_2d(reader, hidden_dim, num_kv_heads * head_dim)?;
        let wo = self.read_tensor_2d(reader, hidden_dim, hidden_dim)?;

        // Layer Norm 1
        let attention_norm_gamma = self.read_tensor_2d(reader, 1, hidden_dim)?;
        let attention_norm_beta = self.read_tensor_2d(reader, 1, hidden_dim)?;

        // FFN 权重
        let w1 = self.read_tensor_2d(reader, hidden_dim, config.ff_dim)?;
        let w3 = self.read_tensor_2d(reader, hidden_dim, config.ff_dim)?;
        let w2 = self.read_tensor_2d(reader, config.ff_dim, hidden_dim)?;

        // Layer Norm 2
        let ffn_norm_gamma = self.read_tensor_2d(reader, 1, hidden_dim)?;
        let ffn_norm_beta = self.read_tensor_2d(reader, 1, hidden_dim)?;

        Ok(LayerWeights {
            wq,
            wk,
            wv,
            wo,
            w1,
            w3,
            w2,
            attention_norm_gamma,
            attention_norm_beta,
            ffn_norm_gamma,
            ffn_norm_beta,
        })
    }

    /// 读取 2D tensor
    fn read_tensor_2d<R: Read>(
        &self,
        reader: &mut R,
        dim0: usize,
        dim1: usize,
    ) -> Result<ndarray::Array2<f32>> {
        let mut data = vec![0f32; dim0 * dim1];

        // 根据量化类型读取
        match self.quantization {
            QuantizationType::F16 => {
                let mut half_data = vec![0u16; dim0 * dim1];
                reader
                    .read_exact(bytemuck::cast_slice_mut(&mut half_data))
                    .map_err(|e| LLMError::LoadError(e.to_string()))?;
                for (i, h) in half_data.iter().enumerate() {
                    data[i] = half_to_float(*h);
                }
            }
            QuantizationType::Q8_0 | QuantizationType::Q4_1 | QuantizationType::Q4_0 => {
                return Err(LLMError::QuantizationError(
                    "Quantized loading not implemented".to_string(),
                ));
            }
            QuantizationType::F32 | QuantizationType::None => {
                reader
                    .read_exact(bytemuck::cast_slice_mut(&mut data))
                    .map_err(|e| LLMError::LoadError(e.to_string()))?;
            }
            QuantizationType::BF16 => {
                let mut bf16_data = vec![0u16; dim0 * dim1];
                reader
                    .read_exact(bytemuck::cast_slice_mut(&mut bf16_data))
                    .map_err(|e| LLMError::LoadError(e.to_string()))?;
                for (i, b) in bf16_data.iter().enumerate() {
                    data[i] = bf16_to_float(*b);
                }
            }
            QuantizationType::Int8
            | QuantizationType::Int4
            | QuantizationType::Int3
            | QuantizationType::FP8 => {
                return Err(LLMError::QuantizationError(
                    "Quantized loading not implemented for this type".to_string(),
                ));
            }
        }

        // reshape 为 2D
        let arr = ndarray::Array2::from_shape_vec((dim0, dim1), data)
            .map_err(|e| LLMError::LoadError(e.to_string()))?;

        Ok(arr)
    }

    /// 从 SafeTensors 格式加载
    fn load_safetensors(&self) -> Result<LoadResult> {
        // SafeTensors 格式加载需要 safetensors crate
        // 这里提供接口，实际实现需要依赖
        Err(LLMError::LoadError(
            "SafeTensors loading requires safetensors crate".to_string(),
        ))
    }

    /// 从 PyTorch 格式加载
    fn load_pytorch(&self) -> Result<LoadResult> {
        // PyTorch 加载需要 torch 或 ndarray-npy
        Err(LLMError::LoadError(
            "PyTorch loading requires torch crate".to_string(),
        ))
    }
}

/// 从 GGML half (f16) 转换
fn half_to_float(bits: u16) -> f32 {
    let sign = (bits >> 15) as f32 * -2.0 + 1.0;
    let exp = ((bits >> 10) & 0x1f) as i32 - 15;
    let mantissa = (bits & 0x3ff) as f32 / 1024.0 + 1.0;

    sign * mantissa * 2.0_f32.powi(exp)
}

/// 从 BF16 转换
fn bf16_to_float(bits: u16) -> f32 {
    let bits = (bits as u32) << 16;
    f32::from_bits(bits)
}

/// 从 HuggingFace 格式加载模型配置
pub fn load_config_json(path: &Path) -> Result<serde_json::Value> {
    let config_path = path
        .parent()
        .map(|p| p.join("config.json"))
        .ok_or_else(|| LLMError::LoadError("Invalid path".to_string()))?;

    let content =
        std::fs::read_to_string(&config_path).map_err(|e| LLMError::LoadError(e.to_string()))?;

    serde_json::from_str(&content).map_err(|e| LLMError::LoadError(e.to_string()))
}

/// 根据配置推断模型类型
pub fn infer_model_type(config: &serde_json::Value) -> Option<ModelType> {
    let model_type = config.get("model_type")?.as_str()?;

    match model_type {
        "Llama" | "llama" => Some(ModelType::LLaMA),
        "GPT2" | "gpt2" => Some(ModelType::GPT2),
        "GPTJ" | "gptj" => Some(ModelType::GPTJ),
        "GPTNeoX" | "gpt-neox" => Some(ModelType::GPTNeoX),
        "Falcon" | "falcon" => Some(ModelType::Falcon),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_conversion() {
        let f16 = 0x3C00; // 1.0 in f16
        let f32 = half_to_float(f16);
        assert!((f32 - 1.0).abs() < 0.01);
    }
}
