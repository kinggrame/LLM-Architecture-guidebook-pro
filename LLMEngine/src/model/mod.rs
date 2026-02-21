//! 模型定义模块
//!
//! 支持多种模型架构：
//! - GPT-2
//! - LLaMA
//! - GPT-J
//! - GPT-NeoX
//! - Falcon

use ndarray::{Array, Array2, Array3, Axis};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 子模块预留（实际实现时可展开）
// pub mod llama;
// pub mod gpt2;
// pub mod gptj;

// pub use llama::LlamaModel;
// pub use gpt2::GPT2Model;
// pub use gptj::GPTJModel;

/// 模型类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelType {
    GPT2,
    GPTJ,
    GPTNeoX,
    LLaMA,
    Falcon,
}

/// 模型配置
#[derive(Debug, Clone)]
pub struct ModelConfig {
    /// 模型类型
    pub model_type: ModelType,
    /// 词汇表大小
    pub vocab_size: usize,
    /// 嵌入维度
    pub embedding_dim: usize,
    /// 注意力头数
    pub num_heads: usize,
    /// Key-Value 头数
    pub num_kv_heads: usize,
    /// 层数
    pub num_layers: usize,
    /// 前馈网络维度
    pub ff_dim: usize,
    /// 最大序列长度
    pub max_seq_len: usize,
    /// 注意力维度
    pub head_dim: usize,
    /// 旋转位置编码维度
    pub rope_dim: Option<usize>,
    /// 词汇表extra IDs 数量
    pub extra_ids: usize,
}

impl ModelConfig {
    /// 从模型类型和参数推断配置
    pub fn from_params(model_type: ModelType, params: &HashMap<String, usize>) -> Self {
        match model_type {
            ModelType::GPT2 => Self {
                model_type,
                vocab_size: *params.get("vocab_size").unwrap_or(&50257),
                embedding_dim: *params.get("n_embd").unwrap_or(&768),
                num_heads: *params.get("n_head").unwrap_or(&12),
                num_kv_heads: *params.get("n_head").unwrap_or(&12),
                num_layers: *params.get("n_layer").unwrap_or(&12),
                ff_dim: *params.get("n_inner").unwrap_or(&3072),
                max_seq_len: *params.get("n_positions").unwrap_or(&1024),
                head_dim: params.get("n_embd").unwrap_or(&768)
                    / params.get("n_head").unwrap_or(&12),
                rope_dim: None,
                extra_ids: 0,
            },
            ModelType::LLaMA => Self {
                model_type,
                vocab_size: *params.get("vocab_size").unwrap_or(&32000),
                embedding_dim: *params.get("hidden_size").unwrap_or(&4096),
                num_heads: *params.get("num_attention_heads").unwrap_or(&32),
                num_kv_heads: *params.get("num_key_value_heads").unwrap_or(&32),
                num_layers: *params.get("num_hidden_layers").unwrap_or(&32),
                ff_dim: *params.get("intermediate_size").unwrap_or(&11008),
                max_seq_len: *params.get("max_position_embeddings").unwrap_or(&2048),
                head_dim: *params.get("hidden_size").unwrap_or(&4096)
                    / params.get("num_attention_heads").unwrap_or(&32),
                rope_dim: params
                    .get("rope_theta")
                    .copied()
                    .or_else(|| params.get("hidden_size").copied()),
                extra_ids: *params.get("extra_ids").unwrap_or(&0),
            },
            ModelType::GPTJ => Self {
                model_type,
                vocab_size: *params.get("vocab_size").unwrap_or(&50400),
                embedding_dim: *params.get("n_embd").unwrap_or(&4096),
                num_heads: *params.get("n_head").unwrap_or(&16),
                num_kv_heads: *params.get("n_head").unwrap_or(&16),
                num_layers: *params.get("n_layer").unwrap_or(&28),
                ff_dim: *params.get("n_inner").unwrap_or(&16384),
                max_seq_len: *params.get("n_positions").unwrap_or(&2048),
                head_dim: 256,
                rope_dim: None,
                extra_ids: 0,
            },
            _ => panic!("Unsupported model type"),
        }
    }
}

/// 模型 trait
pub trait Model: Send + Sync {
    /// 前向传播
    fn forward(
        &self,
        input_ids: &[usize],
        position_ids: Option<&[usize]>,
        kv_cache: Option<&KVCache>,
    ) -> Result<Array2<f32>, crate::LLMError>;

    /// 获取模型配置
    fn config(&self) -> &ModelConfig;

    /// 获取词汇表大小
    fn vocab_size(&self) -> usize;

    /// 获取模型参数数量
    fn num_parameters(&self) -> usize;

    /// 获取模型名称
    fn name(&self) -> &str;
}

/// KV Cache 结构
///
/// 这是推理优化的关键：
/// - 存储之前计算的 Key 和 Value
/// - 避免重复计算
/// - 大大加速自回归生成
#[derive(Clone)]
pub struct KVCache {
    /// Key 缓存: [layer, batch, head, seq_len, head_dim]
    pub keys: Vec<Array3<f32>>,
    /// Value 缓存: [layer, batch, head, seq_len, head_dim]
    pub values: Vec<Array3<f32>>,
    /// 当前序列长度
    pub seq_len: usize,
    /// 最大序列长度
    pub max_seq_len: usize,
    /// 层数
    pub num_layers: usize,
    /// 头数
    pub num_heads: usize,
    /// 头维度
    pub head_dim: usize,
}

impl KVCache {
    /// 创建新的 KV Cache
    pub fn new(num_layers: usize, num_heads: usize, head_dim: usize, max_seq_len: usize) -> Self {
        let keys = (0..num_layers)
            .map(|_| Array3::zeros((num_heads, max_seq_len, head_dim)))
            .collect();

        let values = (0..num_layers)
            .map(|_| Array3::zeros((num_heads, max_seq_len, head_dim)))
            .collect();

        Self {
            keys,
            values,
            seq_len: 0,
            max_seq_len,
            num_layers,
            num_heads,
            head_dim,
        }
    }

    /// 更新缓存
    pub fn update(
        &mut self,
        layer_idx: usize,
        head_idx: usize,
        pos: usize,
        key: &[f32],
        value: &[f32],
    ) {
        if pos < self.max_seq_len {
            for d in 0..self.head_dim {
                self.keys[layer_idx][[head_idx, pos, d]] = key[d];
                self.values[layer_idx][[head_idx, pos, d]] = value[d];
            }
        }
    }

    /// 获取 Key
    pub fn get_key(&self, layer_idx: usize, head_idx: usize, pos: usize) -> Option<Vec<f32>> {
        if pos < self.seq_len {
            let mut result = vec![0.0f32; self.head_dim];
            for d in 0..self.head_dim {
                result[d] = self.keys[layer_idx][[head_idx, pos, d]];
            }
            Some(result)
        } else {
            None
        }
    }

    /// 获取 Value
    pub fn get_value(&self, layer_idx: usize, head_idx: usize, pos: usize) -> Option<Vec<f32>> {
        if pos < self.seq_len {
            let mut result = vec![0.0f32; self.head_dim];
            for d in 0..self.head_dim {
                result[d] = self.values[layer_idx][[head_idx, pos, d]];
            }
            Some(result)
        } else {
            None
        }
    }

    /// 更新序列长度
    pub fn update_seq_len(&mut self, new_len: usize) {
        self.seq_len = new_len.min(self.max_seq_len);
    }

    /// 清空缓存
    pub fn clear(&mut self) {
        self.seq_len = 0;
        for key in &mut self.keys {
            key.fill(0.0);
        }
        for value in &mut self.values {
            value.fill(0.0);
        }
    }
}

/// 注意力计算结果
pub struct AttentionOutput {
    /// 输出: [batch, seq_len, embedding_dim]
    pub output: Array2<f32>,
    /// Key 缓存更新
    pub key_cache: Vec<Array3<f32>>,
    /// Value 缓存更新
    pub value_cache: Vec<Array3<f32>>,
}

/// 模型权重
#[derive(Debug, Clone)]
pub struct ModelWeights {
    /// Token 嵌入: [vocab_size, embedding_dim]
    pub token_embedding: Array2<f32>,
    /// 多个层的权重
    pub layers: Vec<LayerWeights>,
    /// 最终 Layer Norm
    pub final_layernorm_gamma: Array2<f32>,
    pub final_layernorm_beta: Array2<f32>,
    /// LM Head: [vocab_size, embedding_dim] (可能需要转置)
    pub lm_head: Option<Array2<f32>>,
}

/// 单层注意力权重
#[derive(Debug, Clone)]
pub struct LayerWeights {
    /// Query 投影
    pub wq: Array2<f32>,
    /// Key 投影
    pub wk: Array2<f32>,
    /// Value 投影
    pub wv: Array2<f32>,
    /// Output 投影
    pub wo: Array2<f32>,
    /// Feed-Forward 第一个投影
    pub w1: Array2<f32>,
    /// Feed-Forward Gate 投影
    pub w3: Array2<f32>,
    /// Feed-Forward 输出投影
    pub w2: Array2<f32>,
    /// Input Layer Norm
    pub attention_norm_gamma: Array2<f32>,
    pub attention_norm_beta: Array2<f32>,
    /// Post Attention Layer Norm
    pub ffn_norm_gamma: Array2<f32>,
    pub ffn_norm_beta: Array2<f32>,
}

/// RoPE（旋转位置编码）
pub struct RoPE {
    /// 频率
    pub frequencies: Array2<f32>,
}

impl RoPE {
    pub fn new(dim: usize, max_seq_len: usize, theta: f32) -> Self {
        let mut frequencies = Array2::zeros((max_seq_len, dim / 2));

        for pos in 0..max_seq_len {
            for i in 0..(dim / 2) {
                let freq = pos as f32 / theta.powf(2.0 * i as f32 / dim as f32);
                frequencies[[pos, i]] = freq.exp().sin();
                frequencies[[pos, i + dim / 2]] = freq.exp().cos();
            }
        }

        Self { frequencies }
    }

    /// 应用 RoPE 旋转
    pub fn apply(&self, x: &mut [f32], position: usize) {
        let half_dim = x.len() / 2;

        for i in 0..half_dim {
            let x0 = x[i];
            let x1 = x[i + half_dim];
            let freq = self.frequencies[[position, i]];

            x[i] = x0 * freq - x1 * freq;
            x[i + half_dim] = x0 * freq + x1 * freq;
        }
    }
}

/// Layer Norm 实现
pub fn layer_norm(
    x: &Array2<f32>,
    gamma: &Array2<f32>,
    beta: &Array2<f32>,
    eps: f32,
) -> Array2<f32> {
    let seq_len = x.shape()[0];
    let hidden_dim = x.shape()[1];

    let mut output = Array2::zeros((seq_len, hidden_dim));

    for i in 0..seq_len {
        // 计算均值
        let mean = x.row(i).mean().unwrap_or(0.0);

        // 计算方差
        let variance = x.row(i).var(0.0);
        let std = (variance + eps).sqrt();

        // 归一化
        for j in 0..hidden_dim {
            let normalized = (x[[i, j]] - mean) / std;
            output[[i, j]] = normalized * gamma[[0, j]] + beta[[0, j]];
        }
    }

    output
}

/// SwiGLU 激活函数（LLaMA 使用）
pub fn swiglu(
    x: &Array2<f32>,
    w1: &Array2<f32>,
    w3: &Array2<f32>,
    w2: &Array2<f32>,
) -> Array2<f32> {
    // SwiGLU = Swish(x * W1) * (x * W3) * W2
    // 这里简化处理
    let hidden = x.dot(w1);
    let gate = x.dot(w3);

    // SiLU 激活
    let silu = hidden.mapv(|v| v * (1.0 / (1.0 + (-v).exp())));

    // 逐元素相乘
    let gated = silu * gate;

    gated.dot(w2)
}
