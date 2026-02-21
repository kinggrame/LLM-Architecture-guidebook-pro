// src/attention/multi_head_attention.rs - 多头注意力机制
//
// Multi-Head Attention 原理：
// 1. 将 Q, K, V 投影到多个子空间（head）
// 2. 在每个子空间独立计算注意力
// 3. 拼接所有 head 的输出
// 4. 线性变换得到最终输出
//
// 优点：
// - 允许模型同时关注不同位置的信息
// - 不同的 head 可以学习不同的注意力模式

use super::{compute_attention_scores, softmax, AttentionConfig};
use ndarray::{s, Array2, Axis};
use rand::Rng;

/// 多头注意力层
pub struct MultiHeadAttention {
    /// 配置
    config: AttentionConfig,
    /// Query 投影矩阵
    w_q: Array2<f32>,
    /// Key 投影矩阵
    w_k: Array2<f32>,
    /// Value 投影矩阵
    w_v: Array2<f32>,
    /// 输出投影矩阵
    w_o: Array2<f32>,
}

impl MultiHeadAttention {
    /// 创建新的多头注意力层
    pub fn new(config: AttentionConfig) -> Self {
        let mut rng = rand::thread_rng();

        let scale = (2.0 / config.embedding_dim as f32).sqrt();

        let w_q = Array2::from_shape_fn((config.embedding_dim, config.embedding_dim), |_| {
            rng.gen_range(-scale..scale)
        });
        let w_k = Array2::from_shape_fn((config.embedding_dim, config.embedding_dim), |_| {
            rng.gen_range(-scale..scale)
        });
        let w_v = Array2::from_shape_fn((config.embedding_dim, config.embedding_dim), |_| {
            rng.gen_range(-scale..scale)
        });
        let w_o = Array2::from_shape_fn((config.embedding_dim, config.embedding_dim), |_| {
            rng.gen_range(-scale..scale)
        });

        Self {
            config,
            w_q,
            w_k,
            w_v,
            w_o,
        }
    }

    /// 前向传播
    ///
    /// 输入:
    /// - x: (seq_len, embedding_dim) 简化为 2D
    ///
    /// 输出:
    /// - (seq_len, embedding_dim)
    pub fn forward(&self, x: &Array2<f32>) -> Array2<f32> {
        let seq_len = x.shape()[0];
        let embedding_dim = x.shape()[1];

        // 1. 线性投影到 Q, K, V
        let q = x.dot(&self.w_q.t());
        let k = x.dot(&self.w_k.t());
        let v = x.dot(&self.w_v.t());

        // 2. 分割成多个 head 并计算注意力
        let num_heads = self.config.num_heads;
        let head_dim = self.config.head_dim;

        // 存储每个 head 的输出
        let mut head_outputs: Vec<Array2<f32>> = Vec::new();

        for h in 0..num_heads {
            // 提取第 h 个 head 的 Q, K, V
            let start = h * head_dim;
            let end = start + head_dim;

            let q_h: Array2<f32> = q.slice(s![.., start..end]).to_owned();
            let k_h: Array2<f32> = k.slice(s![.., start..end]).to_owned();
            let v_h: Array2<f32> = v.slice(s![.., start..end]).to_owned();

            // 计算注意力分数
            let scores = compute_attention_scores(&q_h, &k_h, None);

            // Softmax
            let attn_weights = softmax(&scores);

            // 加权求和
            let output = attn_weights.dot(&v_h);
            head_outputs.push(output);
        }

        // 3. 拼接所有 head 的输出
        let mut concatenated = Array2::<f32>::zeros((seq_len, embedding_dim));
        for (h, output) in head_outputs.iter().enumerate() {
            let start = h * head_dim;
            let end = start + head_dim;
            for i in 0..seq_len {
                for j in 0..head_dim {
                    concatenated[[i, start + j]] = output[[i, j]];
                }
            }
        }

        // 4. 最终线性变换
        let output = concatenated.dot(&self.w_o.t());

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_head_attention() {
        let config = AttentionConfig::new(64, 4);
        let attention = MultiHeadAttention::new(config);

        // 模拟输入 (seq_len=3, embedding_dim=64)
        let x = Array2::zeros((3, 64));

        let output = attention.forward(&x);

        println!("Output shape: {:?}", output.shape());

        assert_eq!(output.shape()[0], 3);
        assert_eq!(output.shape()[1], 64);
    }
}
