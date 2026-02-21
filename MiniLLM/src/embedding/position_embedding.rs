// src/embedding/position_embedding.rs - 位置编码
//
// 位置编码（Positional Encoding）为序列中的每个位置添加独特的位置信息
// 使模型能够区分不同位置的 token
//
// 两种常见方法：
// 1. 绝对位置编码：Sinusoidal / Learned
// 2. 相对位置编码：Relative Position Bias
//
// 公式（Sinusoidal）：
// PE(pos, 2i) = sin(pos / 10000^(2i/d_model))
// PE(pos, 2i+1) = cos(pos / 10000^(2i/d_model))

use super::Embedding;
use ndarray::Array2;
use rand::Rng;

/// 位置编码类型
#[derive(Debug, Clone)]
pub enum PositionEncodingType {
    /// 可学习的位置编码
    Learned,
    /// 固定的位置编码（Sinusoidal）
    Sinusoidal,
}

/// 位置编码层
pub struct PositionEmbedding {
    /// 编码类型
    encoding_type: PositionEncodingType,
    /// 嵌入维度
    embedding_dim: usize,
    /// 最大序列长度
    max_seq_len: usize,
    /// 可学习的位置嵌入（如果是 Learned 类型）
    embeddings: Option<Array2<f32>>,
}

impl PositionEmbedding {
    /// 创建新的位置编码
    pub fn new(
        encoding_type: PositionEncodingType,
        embedding_dim: usize,
        max_seq_len: usize,
    ) -> Self {
        let embeddings = match &encoding_type {
            PositionEncodingType::Learned => {
                let mut rng = rand::thread_rng();
                let scale = (2.0 / (max_seq_len + embedding_dim) as f32).sqrt();
                Some(Array2::from_shape_fn((max_seq_len, embedding_dim), |_| {
                    rng.gen_range(-scale..scale)
                }))
            }
            PositionEncodingType::Sinusoidal => None,
        };

        Self {
            encoding_type,
            embedding_dim,
            max_seq_len,
            embeddings,
        }
    }

    /// 生成 Sinusoidal 位置编码
    pub fn sinusoidal(&self, seq_len: usize) -> Array2<f32> {
        let mut pe = Array2::<f32>::zeros((seq_len, self.embedding_dim));

        for pos in 0..seq_len {
            for i in (0..self.embedding_dim).step_by(2) {
                let div_term = (10000_f32).powf(i as f32 / self.embedding_dim as f32);

                pe[[pos, i]] = (pos as f32 / div_term).sin();

                if i + 1 < self.embedding_dim {
                    pe[[pos, i + 1]] = (pos as f32 / div_term).cos();
                }
            }
        }

        pe
    }

    /// 扩展位置编码（用于更长序列）
    pub fn extend(&mut self, new_max_len: usize) {
        if new_max_len > self.max_seq_len {
            if let PositionEncodingType::Learned = &self.encoding_type {
                // 扩展可学习嵌入
                let mut rng = rand::thread_rng();
                let scale = (2.0 / (new_max_len + self.embedding_dim) as f32).sqrt();

                let mut new_embeddings =
                    Array2::from_shape_fn((new_max_len, self.embedding_dim), |_| {
                        rng.gen_range(-scale..scale)
                    });

                // 复制旧的嵌入
                if let Some(ref old) = self.embeddings {
                    for i in 0..self.max_seq_len {
                        for j in 0..self.embedding_dim {
                            new_embeddings[[i, j]] = old[[i, j]];
                        }
                    }
                }

                self.embeddings = Some(new_embeddings);
            }

            self.max_seq_len = new_max_len;
        }
    }
}

impl Embedding for PositionEmbedding {
    fn forward(&self, input_ids: &[usize]) -> Array2<f32> {
        let seq_len = input_ids.len();

        match &self.encoding_type {
            PositionEncodingType::Learned => {
                if let Some(ref embeddings) = self.embeddings {
                    let mut output = Array2::<f32>::zeros((seq_len, self.embedding_dim));

                    for (i, &pos) in input_ids.iter().enumerate() {
                        if pos < self.max_seq_len {
                            for j in 0..self.embedding_dim {
                                output[[i, j]] = embeddings[[pos, j]];
                            }
                        }
                    }

                    output
                } else {
                    self.sinusoidal(seq_len)
                }
            }
            PositionEncodingType::Sinusoidal => self.sinusoidal(seq_len),
        }
    }

    fn embedding_dim(&self) -> usize {
        self.embedding_dim
    }

    fn vocab_size(&self) -> usize {
        self.max_seq_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sinusoidal_encoding() {
        let pos_emb = PositionEmbedding::new(PositionEncodingType::Sinusoidal, 64, 512);

        let encoding = pos_emb.sinusoidal(10);

        println!("Shape: ({}, {})", encoding.shape()[0], encoding.shape()[1]);
        println!("First position:\n{:?}", encoding.row(0));

        assert_eq!(encoding.shape()[0], 10);
        assert_eq!(encoding.shape()[1], 64);
    }
}
