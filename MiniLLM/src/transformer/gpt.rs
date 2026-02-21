// src/transformer/gpt.rs - GPT 模型实现
//
// GPT (Generative Pre-trained Transformer) 架构
// - 只使用 Transformer Decoder
// - 堆叠多个 Transformer Layer
// - 因果注意力（Causal Attention）
// - 下一个词预测

use super::transformer_layer::TransformerLayer;
use super::transformer_layer::TransformerLayerConfig;
use crate::embedding::{
    Embedding, EmbeddingConfig, PositionEmbedding, PositionEncodingType, TokenEmbedding,
};
use ndarray::Array2;
use rand::Rng;

/// GPT 模型配置
#[derive(Debug, Clone)]
pub struct GPTConfig {
    /// 词汇表大小
    pub vocab_size: usize,
    /// 嵌入维度
    pub embedding_dim: usize,
    /// 注意力头数
    pub num_heads: usize,
    /// Transformer 层数
    pub num_layers: usize,
    /// 前馈网络维度
    pub ff_dim: usize,
    /// 最大序列长度
    pub max_seq_len: usize,
    /// Dropout
    pub dropout: f32,
}

impl GPTConfig {
    pub fn new(
        vocab_size: usize,
        embedding_dim: usize,
        num_heads: usize,
        num_layers: usize,
        max_seq_len: usize,
    ) -> Self {
        Self {
            vocab_size,
            embedding_dim,
            num_heads,
            num_layers,
            ff_dim: embedding_dim * 4,
            max_seq_len,
            dropout: 0.1,
        }
    }
}

/// GPT 模型
pub struct GPT {
    config: GPTConfig,
    /// Token 嵌入层
    token_embedding: TokenEmbedding,
    /// 位置嵌入层
    position_embedding: PositionEmbedding,
    /// Transformer 层堆叠
    layers: Vec<TransformerLayer>,
    /// 输出层
    lm_head: Vec<Vec<f32>>,
}

impl GPT {
    /// 创建新的 GPT 模型
    pub fn new(
        vocab_size: usize,
        embedding_dim: usize,
        num_heads: usize,
        num_layers: usize,
        max_seq_len: usize,
    ) -> Self {
        let config = GPTConfig::new(
            vocab_size,
            embedding_dim,
            num_heads,
            num_layers,
            max_seq_len,
        );

        // Token 嵌入
        let token_emb_config = EmbeddingConfig {
            vocab_size,
            embedding_dim,
            max_seq_len,
            trainable: true,
        };
        let token_embedding = TokenEmbedding::new(token_emb_config);

        // 位置嵌入
        let position_embedding =
            PositionEmbedding::new(PositionEncodingType::Sinusoidal, embedding_dim, max_seq_len);

        // Transformer 层
        let mut layers = Vec::new();
        for _ in 0..num_layers {
            let layer_config = TransformerLayerConfig::new(embedding_dim, num_heads);
            layers.push(TransformerLayer::new(layer_config));
        }

        // LM Head（语言模型输出层）
        let mut rng = rand::thread_rng();
        let scale = (2.0 / embedding_dim as f32).sqrt();
        let mut lm_head = Vec::new();
        for _ in 0..vocab_size {
            let mut row = Vec::new();
            for _ in 0..embedding_dim {
                row.push(rng.gen_range(-scale..scale));
            }
            lm_head.push(row);
        }

        Self {
            config,
            token_embedding,
            position_embedding,
            layers,
            lm_head,
        }
    }

    /// 前向传播
    pub fn forward(&self, input_ids: &[usize]) -> Vec<Vec<f32>> {
        // 1. Token 嵌入 + 位置嵌入
        let token_emb = self.token_embedding.forward(input_ids);
        let position_ids: Vec<usize> = (0..input_ids.len()).collect();
        let position_emb = self.position_embedding.forward(&position_ids);

        // 相加
        let mut hidden = Array2::zeros(token_emb.raw_dim());
        for i in 0..token_emb.shape()[0] {
            for j in 0..token_emb.shape()[1] {
                hidden[[i, j]] = token_emb[[i, j]] + position_emb[[i, j]];
            }
        }

        // 2. 通过所有 Transformer 层
        for layer in &self.layers {
            hidden = layer.forward(&hidden);
        }

        // 3. LM Head（语言模型头）
        // 简化：直接矩阵乘法
        let vocab_size = self.config.vocab_size;
        let seq_len = input_ids.len();
        let embedding_dim = self.config.embedding_dim;

        let mut logits = Vec::new();

        // 取最后一个位置的输出进行预测
        let last_hidden = hidden.index_axis(ndarray::Axis(0), seq_len - 1);

        for v in 0..vocab_size {
            let mut sum = 0.0;
            for d in 0..embedding_dim {
                sum += last_hidden[d] * self.lm_head[v][d];
            }
            logits.push(sum);
        }

        // Softmax
        let exp_logits: Vec<f32> = logits.iter().map(|&v| v.exp()).collect();
        let sum_exp: f32 = exp_logits.iter().sum();
        let probs: Vec<f32> = exp_logits.iter().map(|&v| v / sum_exp).collect();

        vec![probs]
    }

    /// 生成文本
    pub fn generate(&self, input_ids: &[usize], max_new_tokens: usize) -> Vec<usize> {
        let mut ids = input_ids.to_vec();

        for _ in 0..max_new_tokens {
            // 前向传播
            let probs = self.forward(&ids);

            // 取最后一个 token 的预测
            let last_probs = &probs[probs.len() - 1];

            // 贪心采样
            let mut max_prob = f32::NEG_INFINITY;
            let mut next_token = 0;
            for (i, &prob) in last_probs.iter().enumerate() {
                if prob > max_prob {
                    max_prob = prob;
                    next_token = i;
                }
            }

            ids.push(next_token);
        }

        ids
    }

    /// 获取模型参数量
    pub fn num_parameters(&self) -> usize {
        let mut params = 0;

        // Token embedding
        params += self.config.vocab_size * self.config.embedding_dim;

        // Position embedding
        params += self.config.max_seq_len * self.config.embedding_dim;

        // Transformer layers
        for _ in &self.layers {
            // Attention
            params += self.config.embedding_dim * self.config.embedding_dim * 4;
            // FFN
            params += self.config.embedding_dim * self.config.ff_dim;
            params += self.config.ff_dim * self.config.embedding_dim;
        }

        // LM head
        params += self.config.vocab_size * self.config.embedding_dim;

        params
    }
}
