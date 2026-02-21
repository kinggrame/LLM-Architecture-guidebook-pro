// src/embedding/mod.rs - 词嵌入模块
//
// 本模块实现词嵌入（Word Embedding）
// 包括：Token Embedding, Position Embedding
//
// 词嵌入是将离散符号映射到连续向量空间的技术
// 使得语义相似的词在向量空间中距离相近

mod position_embedding;
mod token_embedding;

pub use position_embedding::{PositionEmbedding, PositionEncodingType};
pub use token_embedding::TokenEmbedding;

use ndarray::Array2;

/// 嵌入层的配置
#[derive(Debug, Clone)]
pub struct EmbeddingConfig {
    /// 词汇表大小
    pub vocab_size: usize,
    /// 嵌入维度
    pub embedding_dim: usize,
    /// 最大序列长度
    pub max_seq_len: usize,
    /// 是否使用可学习的参数
    pub trainable: bool,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            vocab_size: 50000,
            embedding_dim: 768,
            max_seq_len: 512,
            trainable: true,
        }
    }
}

/// 嵌入层 trait
pub trait Embedding {
    /// 前向传播
    fn forward(&self, input_ids: &[usize]) -> Array2<f32>;

    /// 获取嵌入维度
    fn embedding_dim(&self) -> usize;

    /// 获取词汇表大小
    fn vocab_size(&self) -> usize;
}
