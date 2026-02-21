// src/attention/mod.rs - 注意力机制模块
//
// 本模块实现 Transformer 的核心：自注意力机制（Self-Attention）
//
// 注意力机制的核心思想：
// - 决定在处理每个 token 时，应该"关注"输入序列中的哪些部分
// - Query: 当前 token 想查询什么
// - Key: 每个 token 表示"我包含什么信息"
// - Value: 当匹配成功时，返回什么信息
//
// 计算过程：
// Attention(Q, K, V) = softmax(QK^T / sqrt(d_k))V

mod multi_head_attention;
mod scaled_dot_product_attention;

pub use multi_head_attention::MultiHeadAttention;
pub use scaled_dot_product_attention::ScaledDotProductAttention;

use ndarray::{Array2, Axis};

/// 注意力配置
#[derive(Debug, Clone)]
pub struct AttentionConfig {
    /// 嵌入维度
    pub embedding_dim: usize,
    /// 注意力头数
    pub num_heads: usize,
    /// 注意力维度
    pub head_dim: usize,
    /// Dropout 概率
    pub dropout: f32,
    /// 是否使用因果掩码（用于语言模型）
    pub causal: bool,
}

impl AttentionConfig {
    pub fn new(embedding_dim: usize, num_heads: usize) -> Self {
        assert!(
            embedding_dim % num_heads == 0,
            "embedding_dim must be divisible by num_heads"
        );

        Self {
            embedding_dim,
            num_heads,
            head_dim: embedding_dim / num_heads,
            dropout: 0.0,
            causal: true,
        }
    }
}

/// 注意力分数计算
pub fn compute_attention_scores(
    query: &Array2<f32>,
    key: &Array2<f32>,
    mask: Option<&Array2<f32>>,
) -> Array2<f32> {
    // QK^T
    let scores = query.dot(&key.t());

    // 缩放
    let head_dim = query.shape()[1];
    let scale = (head_dim as f32).sqrt();
    let scaled = scores / scale;

    // 应用 mask（如果有）
    if let Some(m) = mask {
        // 将 mask 为 0 的位置设为 -inf
        let mut masked = scaled.clone();
        for ((i, j), &mask_val) in m.indexed_iter() {
            if mask_val == 0.0 {
                masked[[i, j]] = f32::NEG_INFINITY;
            }
        }
        masked
    } else {
        scaled
    }
}

/// Softmax 函数
pub fn softmax(x: &Array2<f32>) -> Array2<f32> {
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
