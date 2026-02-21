// src/embedding/token_embedding.rs - Token 嵌入层
//
// Token Embedding 将 token ID 映射为密集向量
// 这是 Transformer 模型的第一层

use super::{Embedding, EmbeddingConfig};
use ndarray::{Array, Array2, Axis};
use rand::Rng;

/// Token 嵌入层
pub struct TokenEmbedding {
    /// 嵌入矩阵 (vocab_size, embedding_dim)
    embeddings: Array2<f32>,
    /// 配置
    config: EmbeddingConfig,
}

impl TokenEmbedding {
    /// 创建新的嵌入层
    pub fn new(config: EmbeddingConfig) -> Self {
        // 使用 Xavier 初始化
        let mut rng = rand::thread_rng();
        let scale = (2.0 / (config.vocab_size + config.embedding_dim) as f32).sqrt();

        let embeddings = Array::from_shape_fn((config.vocab_size, config.embedding_dim), |_| {
            rng.gen_range(-scale..scale)
        });

        Self { embeddings, config }
    }

    /// 从预训练权重创建
    pub fn from_pretrained(embeddings: Array2<f32>, trainable: bool) -> Self {
        let config = EmbeddingConfig {
            vocab_size: embeddings.shape()[0],
            embedding_dim: embeddings.shape()[1],
            max_seq_len: 512,
            trainable,
        };

        Self { embeddings, config }
    }

    /// 获取指定 token 的嵌入向量
    pub fn get_embedding(&self, token_id: usize) -> Option<Array2<f32>> {
        if token_id >= self.config.vocab_size {
            return None;
        }

        // 返回 (1, embedding_dim) 的矩阵
        Some(
            self.embeddings
                .index_axis(Axis(0), token_id)
                .to_owned()
                .into_shape((1, self.config.embedding_dim))
                .unwrap(),
        )
    }

    /// 获取所有 token 的嵌入
    pub fn get_all_embeddings(&self) -> &Array2<f32> {
        &self.embeddings
    }
}

impl Embedding for TokenEmbedding {
    /// 前向传播：将 token IDs 转换为嵌入向量
    ///
    /// 输入: [batch_size, seq_len] 的 token IDs
    /// 输出: [batch_size, seq_len, embedding_dim] 的嵌入向量
    fn forward(&self, input_ids: &[usize]) -> Array2<f32> {
        let seq_len = input_ids.len();
        let embedding_dim = self.config.embedding_dim;

        // 创建输出数组
        let mut output = Array2::<f32>::zeros((seq_len, embedding_dim));

        for (i, &token_id) in input_ids.iter().enumerate() {
            if token_id < self.config.vocab_size {
                // 获取该 token 的嵌入向量
                let embedding = self.embeddings.index_axis(Axis(0), token_id);
                // 复制到输出
                for j in 0..embedding_dim {
                    output[[i, j]] = embedding[j];
                }
            }
        }

        output
    }

    fn embedding_dim(&self) -> usize {
        self.config.embedding_dim
    }

    fn vocab_size(&self) -> usize {
        self.config.vocab_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_embedding() {
        let config = EmbeddingConfig {
            vocab_size: 1000,
            embedding_dim: 64,
            max_seq_len: 512,
            trainable: true,
        };

        let embedding = TokenEmbedding::new(config);

        let input_ids = vec![0, 1, 2, 3, 4];
        let output = embedding.forward(&input_ids);

        println!(
            "Input shape: ({}, {})",
            output.shape()[0],
            output.shape()[1]
        );

        assert_eq!(output.shape()[0], 5);
        assert_eq!(output.shape()[1], 64);
    }
}
