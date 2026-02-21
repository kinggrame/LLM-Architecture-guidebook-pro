//! 推理引擎模块
//!
//! 包含完整的推理流程：
//! - 前向传播
//! - KV Cache 管理
//! - 采样策略
//! - 批量处理

use crate::model::{KVCache, Model, ModelConfig};
use crate::Result;
use ndarray::Array2;
use std::sync::Arc;

/// 推理引擎
pub struct InferenceEngine<M: Model> {
    model: Arc<M>,
    config: crate::InferenceConfig,
    kv_cache: Option<KVCache>,
}

impl<M: Model> InferenceEngine<M> {
    /// 创建新的推理引擎
    pub fn new(model: M, config: crate::InferenceConfig) -> Self {
        let model = Arc::new(model);

        // 初始化 KV Cache
        let kv_cache = if config.use_kv_cache {
            let cfg = model.config();
            Some(KVCache::new(
                cfg.num_layers,
                cfg.num_heads,
                cfg.head_dim,
                cfg.max_seq_len,
            ))
        } else {
            None
        };

        Self {
            model,
            config,
            kv_cache,
        }
    }

    /// 生成文本
    pub fn generate(&mut self, prompt: &[usize], max_new_tokens: usize) -> Result<Vec<usize>> {
        let mut all_tokens = prompt.to_vec();
        let mut current_length = all_tokens.len();

        for _ in 0..max_new_tokens {
            // 截断输入
            if current_length > self.model.config().max_seq_len {
                all_tokens =
                    all_tokens[current_length - self.model.config().max_seq_len..].to_vec();
                current_length = self.model.config().max_seq_len;

                if let Some(ref mut cache) = self.kv_cache {
                    cache.clear();
                }
            }

            // 前向传播
            let logits = self.forward(&all_tokens, current_length)?;

            // 采样下一个 token - logits is [batch, vocab_size], take last row
            let last_row = logits.row(current_length - 1);
            let logits_1d = last_row.to_vec();
            let next_token = self.sample(&logits_1d);

            // 检查结束
            if next_token == self.model.config().vocab_size - 1 {
                break;
            }

            all_tokens.push(next_token);
            current_length += 1;
        }

        Ok(all_tokens[prompt.len()..].to_vec())
    }

    /// 前向传播
    fn forward(&mut self, input_ids: &[usize], position: usize) -> Result<Array2<f32>> {
        // 更新 position IDs
        let position_ids: Vec<usize> = (0..input_ids.len()).collect();

        // 使用 KV Cache
        let kv_cache_ref = self.kv_cache.as_ref();

        self.model
            .forward(input_ids, Some(&position_ids), kv_cache_ref)
    }

    /// 采样
    fn sample(&self, logits: &[f32]) -> usize {
        // 计算 softmax
        let max_val = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let probs: Vec<f32> = logits.iter().map(|v| (v - max_val).exp()).collect();
        let sum: f32 = probs.iter().sum();
        let probs: Vec<f32> = probs.iter().map(|v| v / sum).collect();

        // Top-k 过滤
        let mut probs_vec: Vec<(usize, f32)> =
            probs.iter().enumerate().map(|(i, &p)| (i, p)).collect();

        probs_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let top_k = self.config.top_k.min(probs_vec.len());
        let top_probs: Vec<f32> = probs_vec[..top_k].iter().map(|(_, p)| *p).collect();
        let top_sum: f32 = top_probs.iter().sum();
        let top_probs: Vec<f32> = top_probs.iter().map(|p| p / top_sum).collect();

        // Top-p (nucleus) 过滤
        let mut cumsum = 0.0f32;
        let mut filtered: Vec<(usize, f32)> = Vec::new();
        for (i, &p) in top_probs.iter().enumerate() {
            cumsum += p;
            filtered.push((probs_vec[i].0, p));
            if cumsum >= self.config.top_p {
                break;
            }
        }

        // 归一化
        let sum: f32 = filtered.iter().map(|(_, p)| p).sum();
        let filtered: Vec<(usize, f32)> = filtered.into_iter().map(|(i, p)| (i, p / sum)).collect();

        // 随机采样
        let r: f32 = rand::random();
        let mut cumsum = 0.0f32;
        for (idx, prob) in filtered.iter() {
            cumsum += prob;
            if r <= cumsum {
                return *idx;
            }
        }

        filtered.last().map(|(idx, _)| *idx).unwrap_or(0)
    }

    /// 批量生成
    pub fn batch_generate(
        &mut self,
        prompts: &[Vec<usize>],
        max_new_tokens: usize,
    ) -> Result<Vec<Vec<usize>>> {
        // 简化实现
        let mut results = Vec::new();

        for prompt in prompts {
            let result = self.generate(prompt, max_new_tokens)?;
            results.push(result);
        }

        Ok(results)
    }

    /// 重置 KV Cache
    pub fn reset_cache(&mut self) {
        if let Some(ref mut cache) = self.kv_cache {
            cache.clear();
        }
    }
}

/// 推理会话
pub struct InferenceSession {
    pub input_ids: Vec<usize>,
    pub output_ids: Vec<usize>,
    pub kv_cache: Option<KVCache>,
}

impl InferenceSession {
    pub fn new() -> Self {
        Self {
            input_ids: Vec::new(),
            output_ids: Vec::new(),
            kv_cache: None,
        }
    }

    pub fn with_kv_cache(config: &ModelConfig) -> Self {
        Self {
            input_ids: Vec::new(),
            output_ids: Vec::new(),
            kv_cache: Some(KVCache::new(
                config.num_layers,
                config.num_heads,
                config.head_dim,
                config.max_seq_len,
            )),
        }
    }

    pub fn reset(&mut self) {
        self.input_ids.clear();
        self.output_ids.clear();
        if let Some(ref mut cache) = self.kv_cache {
            cache.clear();
        }
    }
}

impl Default for InferenceSession {
    fn default() -> Self {
        Self::new()
    }
}

/// 采样器
pub struct Sampler {
    pub temperature: f32,
    pub top_k: usize,
    pub top_p: f32,
    pub repeat_penalty: f32,
}

impl Sampler {
    pub fn new(temperature: f32, top_k: usize, top_p: f32, repeat_penalty: f32) -> Self {
        Self {
            temperature,
            top_k,
            top_p,
            repeat_penalty,
        }
    }

    pub fn sample(&self, logits: &mut [f32]) -> usize {
        // 应用温度
        if self.temperature != 1.0 {
            for logit in logits.iter_mut() {
                *logit /= self.temperature;
            }
        }

        // 应用重复惩罚
        // (简化实现)

        // Softmax
        let max_logit = logits.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let exp_logits: Vec<f32> = logits.iter().map(|v| (v - max_logit).exp()).collect();
        let sum: f32 = exp_logits.iter().sum();
        let probs: Vec<f32> = exp_logits.iter().map(|v| v / sum).collect();

        // Top-k + Top-p 过滤
        // ... (简化实现)

        // 采样
        let r: f32 = rand::random();
        let mut cumsum = 0.0f32;
        for (i, &prob) in probs.iter().enumerate() {
            cumsum += prob;
            if r <= cumsum {
                return i;
            }
        }

        probs.len() - 1
    }
}

impl Default for Sampler {
    fn default() -> Self {
        Self::new(1.0, 40, 0.9, 1.0)
    }
}
