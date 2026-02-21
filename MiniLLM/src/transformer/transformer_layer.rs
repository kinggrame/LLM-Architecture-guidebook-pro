// src/transformer/transformer_layer.rs - Transformer 层
//
// Transformer Layer = MultiHeadAttention + AddNorm + FeedForward + AddNorm
//
// 原始 Transformer 论文中的 Encoder/Decoder 层
// GPT 只使用解码器部分

use crate::attention::AttentionConfig;
use crate::attention::MultiHeadAttention;
use ndarray::{Array1, Array2};
use rand::Rng;

/// Transformer 层配置
#[derive(Debug, Clone)]
pub struct TransformerLayerConfig {
    pub embedding_dim: usize,
    pub num_heads: usize,
    pub ff_dim: usize, // 前馈网络中间层维度
    pub dropout: f32,
}

impl TransformerLayerConfig {
    pub fn new(embedding_dim: usize, num_heads: usize) -> Self {
        Self {
            embedding_dim,
            num_heads,
            ff_dim: embedding_dim * 4,
            dropout: 0.1,
        }
    }
}

/// Transformer 层
pub struct TransformerLayer {
    config: TransformerLayerConfig,
    attention: MultiHeadAttention,
    /// 前馈网络
    w1: Array2<f32>,
    w2: Array2<f32>,
    /// Layer Norm
    gamma1: Array1<f32>,
    beta1: Array1<f32>,
    gamma2: Array1<f32>,
    beta2: Array1<f32>,
}

impl TransformerLayer {
    pub fn new(config: TransformerLayerConfig) -> Self {
        let attention_config = AttentionConfig::new(config.embedding_dim, config.num_heads);
        let attention = MultiHeadAttention::new(attention_config);

        let mut rng = rand::thread_rng();

        // FFN 权重
        let scale = (2.0 / config.embedding_dim as f32).sqrt();
        let w1 = Array2::from_shape_fn((config.embedding_dim, config.ff_dim), |_| {
            rng.gen_range(-scale..scale)
        });
        let w2 = Array2::from_shape_fn((config.ff_dim, config.embedding_dim), |_| {
            rng.gen_range(-scale..scale)
        });

        // Layer Norm 参数
        let gamma1 = Array1::ones(config.embedding_dim);
        let beta1 = Array1::zeros(config.embedding_dim);
        let gamma2 = Array1::ones(config.embedding_dim);
        let beta2 = Array1::zeros(config.embedding_dim);

        Self {
            config,
            attention,
            w1,
            w2,
            gamma1,
            beta1,
            gamma2,
            beta2,
        }
    }

    /// 前向传播
    pub fn forward(&self, x: &Array2<f32>) -> Array2<f32> {
        // 1. Multi-Head Attention + Add + LayerNorm
        let attn_output = self.attention.forward(x);
        let x = x + &attn_output; // Add
        let x = self.layer_norm(&x, &self.gamma1, &self.beta1);

        // 2. Feed Forward + Add + LayerNorm
        let ff_output = self.forward_layer(&x);
        let x = x + &ff_output; // Add
        let x = self.layer_norm(&x, &self.gamma2, &self.beta2);

        x
    }

    /// 前馈网络
    fn forward_layer(&self, x: &Array2<f32>) -> Array2<f32> {
        let hidden = x.dot(&self.w1);
        let hidden = hidden.mapv(|v| v.max(0.0)); // ReLU
        hidden.dot(&self.w2)
    }

    /// Layer Normalization
    fn layer_norm(&self, x: &Array2<f32>, gamma: &Array1<f32>, beta: &Array1<f32>) -> Array2<f32> {
        let eps = 1e-5;
        let mean = x.mean_axis(ndarray::Axis(1)).unwrap();
        let variance = x.var_axis(ndarray::Axis(1), 0.0);

        let std = (variance + eps).mapv(|v| v.sqrt());

        let mut output = Array2::zeros(x.raw_dim());

        for i in 0..x.shape()[0] {
            for j in 0..x.shape()[1] {
                let normalized = (x[[i, j]] - mean[[i]]) / std[[i]];
                output[[i, j]] = normalized * gamma[[j]] + beta[[j]];
            }
        }

        output
    }
}
