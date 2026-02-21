// src/attention/scaled_dot_product_attention.rs - 缩放点积注意力
//
// Scaled Dot-Product Attention 公式：
// Attention(Q, K, V) = softmax(QK^T / sqrt(d_k))V
//
// 其中：
// - Q (Query): 查询向量
// - K (Key): 键向量
// - V (Value): 值向量
// - d_k: Key 的维度

use ndarray::{Array2, Axis};

/// 缩放点积注意力
pub struct ScaledDotProductAttention {
    /// 是否使用因果掩码（用于自回归生成）
    causal: bool,
    /// Dropout 概率（训练时）
    dropout: f32,
}

impl ScaledDotProductAttention {
    /// 创建新的缩放点积注意力层
    pub fn new(causal: bool, dropout: f32) -> Self {
        Self { causal, dropout }
    }

    /// 前向传播
    ///
    /// # 参数
    /// - query: (batch, seq_len_q, d_k)
    /// - key: (batch, seq_len_k, d_k)
    /// - value: (batch, seq_len_k, d_v)
    /// - mask: 可选的注意力掩码
    ///
    /// # 返回
    /// - (batch, seq_len_q, d_v)
    pub fn forward(
        &self,
        query: &Array2<f32>,
        key: &Array2<f32>,
        value: &Array2<f32>,
        mask: Option<&Array2<f32>>,
    ) -> Array2<f32> {
        let d_k = query.shape()[query.shape().len() - 1] as f32;

        // QK^T / sqrt(d_k)
        let scores = query.dot(&key.t()) / d_k.sqrt();

        // 应用因果掩码
        let scores = if self.causal {
            self.apply_causal_mask(&scores)
        } else if let Some(m) = mask {
            self.apply_mask(&scores, m)
        } else {
            scores
        };

        // Softmax
        let attn_weights = softmax(&scores);

        // 注意力加权
        attn_weights.dot(value)
    }

    /// 应用因果掩码（下三角）
    fn apply_causal_mask(&self, scores: &Array2<f32>) -> Array2<f32> {
        let mut masked = scores.clone();
        let seq_len = scores.shape()[0];

        for i in 0..seq_len {
            for j in (i + 1)..seq_len {
                masked[[i, j]] = f32::NEG_INFINITY;
            }
        }

        masked
    }

    /// 应用自定义掩码
    fn apply_mask(&self, scores: &Array2<f32>, mask: &Array2<f32>) -> Array2<f32> {
        let mut masked = scores.clone();

        for ((i, j), &mask_val) in mask.indexed_iter() {
            if mask_val == 0.0 {
                masked[[i, j]] = f32::NEG_INFINITY;
            }
        }

        masked
    }
}

/// Softmax 函数
fn softmax(x: &Array2<f32>) -> Array2<f32> {
    // 数值稳定的 softmax
    let x_max = x.map_axis(Axis(1), |row| {
        row.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
    });

    let mut exp_x = x.clone();
    for i in 0..x.shape()[0] {
        for j in 0..x.shape()[1] {
            exp_x[[i, j]] = (x[[i, j]] - x_max[i]).exp();
        }
    }

    let sum: Vec<f32> = (0..x.shape()[0])
        .map(|i| (0..x.shape()[1]).map(|j| exp_x[[i, j]]).sum())
        .collect();

    for i in 0..x.shape()[0] {
        for j in 0..x.shape()[1] {
            exp_x[[i, j]] /= sum[i];
        }
    }

    exp_x
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array;

    #[test]
    fn test_scaled_dot_product_attention() {
        let attention = ScaledDotProductAttention::new(false, 0.0);

        let q = Array::zeros((3, 4));
        let k = Array::zeros((3, 4));
        let v = Array::zeros((3, 4));

        let output = attention.forward(&q, &k, &v, None);

        assert_eq!(output.shape(), &[3, 4]);
    }
}
