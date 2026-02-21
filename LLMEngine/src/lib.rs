//! LLM Engine - 高性能 LLM 推理引擎
//!
//! 这是一个生产级别的 LLM 推理引擎，支持：
//! - 多种模型架构（GPT, LLaMA, GPT-J, GPT-NeoX）
//! - 量化推理（INT8, INT4）
//! - KV Cache 优化
//! - 批量推理
//! - 多线程并行

pub mod inference;
pub mod loader;
pub mod model;
pub mod quantization;
pub mod tokenizer;
pub mod utils;

pub use inference::{InferenceEngine, InferenceSession, Sampler};
pub use loader::{LoadResult, ModelLoader};
pub use model::{Model, ModelConfig, ModelType};
pub use quantization::QuantizationType;

use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LLMError {
    #[error("Failed to load model: {0}")]
    LoadError(String),

    #[error("Inference error: {0}")]
    InferenceError(String),

    #[error("Tokenizer error: {0}")]
    TokenizerError(String),

    #[error("Quantization error: {0}")]
    QuantizationError(String),

    #[error("Unsupported model: {0}")]
    UnsupportedModel(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, LLMError>;

/// 推理配置
#[derive(Debug, Clone)]
pub struct InferenceConfig {
    /// 最大 token 长度
    pub max_length: usize,
    /// 温度（用于采样）
    pub temperature: f32,
    /// Top-k 采样
    pub top_k: usize,
    /// Top-p 采样（核采样）
    pub top_p: f32,
    /// 重复惩罚
    pub repeat_penalty: f32,
    /// 是否使用 KV cache
    pub use_kv_cache: bool,
    /// 批量大小
    pub batch_size: usize,
    /// 线程数
    pub num_threads: usize,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            max_length: 2048,
            temperature: 0.7,
            top_k: 40,
            top_p: 0.9,
            repeat_penalty: 1.1,
            use_kv_cache: true,
            batch_size: 1,
            num_threads: 4,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_default_config() {
        let config = super::InferenceConfig::default();
        assert_eq!(config.max_length, 2048);
        assert_eq!(config.temperature, 0.7);
    }
}
