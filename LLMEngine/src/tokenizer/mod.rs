//! Tokenizer 模块
//!
//! 支持多种分词器：
//! - BPE（GPT-2, LLaMA 使用）
//! - WordPiece（BERT 使用）
//! - Unigram（中文、日文等）
//!
//! ## 核心概念
//!
//! Tokenizer 是 LLM 的第一步：
//! 1. 将文本转换为 token 序列
//! 2. 将 token 序列转换为 ID
//! 3. 生成 attention mask 和 position IDs
//!
//! ## 分词器格式
//!
//! - HuggingFace tokenizer.json
//! - GGML 格式
//! - SentencePiece 模型

use crate::{LLMError, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::Path;

/// Token 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenizerType {
    BPE,
    WordPiece,
    Unigram,
    Char,
}

/// Tokenizer 配置
#[derive(Debug, Clone)]
pub struct TokenizerConfig {
    /// 分词器类型
    pub tokenizer_type: TokenizerType,
    /// 词汇表大小
    pub vocab_size: usize,
    /// 特殊 token ID
    pub pad_token_id: Option<usize>,
    pub bos_token_id: Option<usize>,
    pub eos_token_id: Option<usize>,
    pub unk_token_id: Option<usize>,
    pub sep_token_id: Option<usize>,
    pub cls_token_id: Option<usize>,
    pub mask_token_id: Option<usize>,
    /// 额外 token 数量（用于聊天模板）
    pub extra_ids: usize,
}

impl Default for TokenizerConfig {
    fn default() -> Self {
        Self {
            tokenizer_type: TokenizerType::BPE,
            vocab_size: 50257,
            pad_token_id: None,
            bos_token_id: Some(1),
            eos_token_id: Some(2),
            unk_token_id: Some(0),
            sep_token_id: None,
            cls_token_id: None,
            mask_token_id: None,
            extra_ids: 0,
        }
    }
}

/// 分词结果
#[derive(Debug, Clone)]
pub struct TokenizeResult {
    /// Token IDs
    pub input_ids: Vec<usize>,
    /// Attention mask（可选）
    pub attention_mask: Option<Vec<usize>>,
    /// Position IDs（可选）
    pub position_ids: Option<Vec<usize>>,
    /// Token 类型 ID（可选）
    pub token_type_ids: Option<Vec<usize>>,
}

/// Tokenizer trait
pub trait Tokenizer: Send + Sync {
    /// 分词
    fn encode(&self, text: &str) -> Result<Vec<usize>>;

    /// 分词（带配置）
    fn encode_with_config(&self, text: &str, add_special_tokens: bool) -> Result<Vec<usize>>;

    /// 解码
    fn decode(&self, ids: &[usize]) -> Result<String>;

    /// 解码（跳过特殊 token）
    fn decode_skip_special_tokens(&self, ids: &[usize]) -> Result<String>;

    /// 获取词汇表大小
    fn vocab_size(&self) -> usize;

    /// 获取配置
    fn config(&self) -> &TokenizerConfig;
}

/// BPE Tokenizer 实现
pub struct BPETokenizer {
    /// 词汇表：token -> id
    vocab: HashMap<String, usize>,
    /// 逆词汇表：id -> token
    inverse_vocab: Vec<String>,
    /// 合并规则（按优先级排序）
    merges: Vec<(String, String)>,
    /// 配置
    config: TokenizerConfig,
    /// 字节级词汇表
    byte_vocab: HashMap<u8, String>,
}

impl BPETokenizer {
    /// 从文件加载
    pub fn from_files(vocab_path: &Path, merges_path: &Path) -> Result<Self> {
        let vocab = Self::load_vocab(vocab_path)?;
        let merges = Self::load_merges(merges_path)?;

        let vocab_size = vocab.len();
        let mut inverse_vocab = vec![String::new(); vocab_size];

        for (token, id) in &vocab {
            if *id < vocab_size {
                inverse_vocab[*id] = token.clone();
            }
        }

        let byte_vocab = Self::build_byte_vocab();

        let mut config = TokenizerConfig::default();
        config.vocab_size = vocab_size;

        Ok(Self {
            vocab,
            inverse_vocab,
            merges,
            config,
            byte_vocab,
        })
    }

    /// 从 HuggingFace tokenizer.json 加载
    pub fn from_tokenizer_json(path: &Path) -> Result<Self> {
        let content =
            fs::read_to_string(path).map_err(|e| LLMError::TokenizerError(e.to_string()))?;

        let json: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| LLMError::TokenizerError(e.to_string()))?;

        // 解析 model
        let model = json.get("model").ok_or_else(|| {
            LLMError::TokenizerError("Missing model in tokenizer.json".to_string())
        })?;

        // 解析 vocab
        let vocab_obj = model
            .get("vocab")
            .and_then(|v| v.as_object())
            .ok_or_else(|| LLMError::TokenizerError("Missing vocab".to_string()))?;

        let mut vocab: HashMap<String, usize> = HashMap::new();
        for (token, id) in vocab_obj {
            if let Some(id) = id.as_u64() {
                vocab.insert(token.clone(), id as usize);
            }
        }

        // 解析 merges
        let mut merges = Vec::new();
        if let Some(merges_arr) = model.get("merges").and_then(|m| m.as_array()) {
            for merge in merges_arr {
                if let Some(m) = merge.as_str() {
                    let parts: Vec<&str> = m.splitn(2, ' ').collect();
                    if parts.len() == 2 {
                        merges.push((parts[0].to_string(), parts[1].to_string()));
                    }
                }
            }
        }

        let vocab_size = vocab.len();
        let mut inverse_vocab = vec![String::new(); vocab_size];
        for (token, id) in &vocab {
            if *id < vocab_size {
                inverse_vocab[*id] = token.clone();
            }
        }

        let byte_vocab = Self::build_byte_vocab();

        // 解析 special tokens
        let mut config = TokenizerConfig::default();
        config.vocab_size = vocab_size;

        if let Some(added_tokens) = json.get("added_tokens").and_then(|t| t.as_array()) {
            for token in added_tokens {
                if let (Some(id), Some(token_str), Some(special)) = (
                    token.get("id").and_then(|i| i.as_u64()),
                    token.get("content").and_then(|c| c.as_str()),
                    token.get("special").and_then(|s| s.as_bool()),
                ) {
                    if special {
                        if token_str == "<pad>" {
                            config.pad_token_id = Some(id as usize);
                        } else if token_str == "<s>" || token_str == "<bos>" {
                            config.bos_token_id = Some(id as usize);
                        } else if token_str == "</s>" || token_str == "<eos>" {
                            config.eos_token_id = Some(id as usize);
                        } else if token_str == "<unk>" {
                            config.unk_token_id = Some(id as usize);
                        }
                    }
                }
            }
        }

        Ok(Self {
            vocab,
            inverse_vocab,
            merges,
            config,
            byte_vocab,
        })
    }

    /// 加载词汇表
    fn load_vocab(path: &Path) -> Result<HashMap<String, usize>> {
        let content =
            fs::read_to_string(path).map_err(|e| LLMError::TokenizerError(e.to_string()))?;

        let mut vocab = HashMap::new();

        for (id, line) in content.lines().enumerate() {
            vocab.insert(line.to_string(), id);
        }

        Ok(vocab)
    }

    /// 加载合并规则
    fn load_merges(path: &Path) -> Result<Vec<(String, String)>> {
        let content =
            fs::read_to_string(path).map_err(|e| LLMError::TokenizerError(e.to_string()))?;

        let mut merges = Vec::new();

        for line in content.lines() {
            if line.contains(' ') {
                let parts: Vec<&str> = line.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    merges.push((parts[0].to_string(), parts[1].to_string()));
                }
            }
        }

        Ok(merges)
    }

    /// 构建字节级词汇表
    fn build_byte_vocab() -> HashMap<u8, String> {
        let mut byte_vocab = HashMap::new();

        // ASCII 字符
        for i in 0..128u8 {
            byte_vocab.insert(i, (i as char).to_string());
        }

        // 扩展 Unicode
        for i in 128..=255u8 {
            byte_vocab.insert(i, format!("<0x{:02X}>", i));
        }

        byte_vocab
    }

    /// 将字符串转换为字节
    fn text_to_bytes(&self, text: &str) -> Vec<u8> {
        text.bytes().collect()
    }

    /// BPE 分词
    fn bpe_tokenize(&self, word: &str) -> Vec<String> {
        // 字节级编码
        let mut word_bytes: Vec<String> = self
            .text_to_bytes(word)
            .iter()
            .map(|b| {
                self.byte_vocab
                    .get(b)
                    .cloned()
                    .unwrap_or_else(|| format!("<0x{:02X}>", b))
            })
            .collect();

        // 应用合并规则
        let mut new_tokens = true;
        while new_tokens {
            new_tokens = false;

            for (first, second) in &self.merges {
                let mut i = 0;
                while i + 1 < word_bytes.len() {
                    if &word_bytes[i] == first && &word_bytes[i + 1] == second {
                        // 合并
                        let merged = format!("{}{}", first, second);
                        word_bytes[i] = merged;
                        word_bytes.remove(i + 1);
                        new_tokens = true;
                    } else {
                        i += 1;
                    }
                }
            }
        }

        word_bytes
    }
}

impl Tokenizer for BPETokenizer {
    fn encode(&self, text: &str) -> Result<Vec<usize>> {
        self.encode_with_config(text, true)
    }

    fn encode_with_config(&self, text: &str, add_special_tokens: bool) -> Result<Vec<usize>> {
        let mut tokens = Vec::new();

        // 添加 BOS
        if add_special_tokens {
            if let Some(bos_id) = self.config.bos_token_id {
                tokens.push(bos_id);
            }
        }

        // 分词
        for word in text.split_whitespace() {
            let word_tokens = self.bpe_tokenize(word);

            for token in &word_tokens {
                if let Some(&id) = self.vocab.get(token) {
                    tokens.push(id);
                } else if let Some(unk_id) = self.config.unk_token_id {
                    tokens.push(unk_id);
                }
            }
        }

        // 添加 EOS
        if add_special_tokens {
            if let Some(eos_id) = self.config.eos_token_id {
                tokens.push(eos_id);
            }
        }

        Ok(tokens)
    }

    fn decode(&self, ids: &[usize]) -> Result<String> {
        let mut text = String::new();

        for id in ids {
            if let Some(token) = self.inverse_vocab.get(*id) {
                // 处理字节级 token
                if token.starts_with("<0x") && token.ends_with('}') {
                    if let Ok(byte) = u8::from_str_radix(&token[3..5], 16) {
                        text.push(byte as char);
                    }
                } else {
                    text.push_str(token);
                }
            }
        }

        Ok(text)
    }

    fn decode_skip_special_tokens(&self, ids: &[usize]) -> Result<String> {
        let special_ids: std::collections::HashSet<usize> = [
            self.config.pad_token_id,
            self.config.bos_token_id,
            self.config.eos_token_id,
            self.config.unk_token_id,
            self.config.sep_token_id,
            self.config.cls_token_id,
            self.config.mask_token_id,
        ]
        .into_iter()
        .flatten()
        .collect();

        let filtered: Vec<usize> = ids
            .iter()
            .filter(|id| !special_ids.contains(*id))
            .copied()
            .collect();

        self.decode(&filtered)
    }

    fn vocab_size(&self) -> usize {
        self.config.vocab_size
    }

    fn config(&self) -> &TokenizerConfig {
        &self.config
    }
}

/// 创建 GPT-2 tokenizer
pub fn create_gpt2_tokenizer() -> Result<BPETokenizer> {
    // 默认 GPT-2 词汇表
    let mut vocab: HashMap<String, usize> = HashMap::new();

    // 基础 token
    let special_tokens = [
        "<|endoftext|>",
        "<|pad|>",
        "<|startoftranscript|>",
        "<|endoftranscript|>",
    ];
    for (i, token) in special_tokens.iter().enumerate() {
        vocab.insert(token.to_string(), i);
    }

    // 填充 ASCII 和扩展
    for i in 0..256 {
        vocab.insert(format!("<0x{:02X}>", i), vocab.len());
    }

    // 常见词（简化）
    for i in b'!'..=b'z' {
        let s = (i as char).to_string();
        if !vocab.contains_key(&s) {
            vocab.insert(s, vocab.len());
        }
    }

    let vocab_size = vocab.len();
    let mut inverse_vocab = vec![String::new(); vocab_size];
    for (token, id) in &vocab {
        if *id < vocab_size {
            inverse_vocab[*id] = token.clone();
        }
    }

    let byte_vocab = BPETokenizer::build_byte_vocab();

    let mut config = TokenizerConfig::default();
    config.vocab_size = vocab_size;
    config.bos_token_id = None;
    config.eos_token_id = Some(0); // <|endoftext|>
    config.pad_token_id = Some(1);

    Ok(BPETokenizer {
        vocab,
        inverse_vocab,
        merges: Vec::new(),
        config,
        byte_vocab,
    })
}

/// 创建 LLaMA tokenizer
pub fn create_llama_tokenizer() -> Result<BPETokenizer> {
    // 简化版 LLaMA tokenizer
    let mut vocab: HashMap<String, usize> = HashMap::new();

    // 特殊 token
    let special_tokens = [
        "<unk>",
        "<s>",
        "</s>",
        "<|pad|>",
        "<|bos|>",
        "<|eos|>",
        "<|system|>",
        "<|user|>",
        "<|assistant|>",
        "<|observ|>",
        "<|action|>",
        "<|result|>",
    ];
    for (i, token) in special_tokens.iter().enumerate() {
        vocab.insert(token.to_string(), i);
    }

    // 字节级
    for i in 0..256 {
        vocab.insert(format!("<0x{:02X}>", i), vocab.len());
    }

    // 控制符
    for i in 0..32 {
        vocab.insert(format!("<0x{:02X}>", i), vocab.len());
    }

    let vocab_size = vocab.len();
    let mut inverse_vocab = vec![String::new(); vocab_size];
    for (token, id) in &vocab {
        if *id < vocab_size {
            inverse_vocab[*id] = token.clone();
        }
    }

    let byte_vocab = BPETokenizer::build_byte_vocab();

    let mut config = TokenizerConfig::default();
    config.vocab_size = vocab_size;
    config.bos_token_id = Some(1);
    config.eos_token_id = Some(2);
    config.unk_token_id = Some(0);

    Ok(BPETokenizer {
        vocab,
        inverse_vocab,
        merges: Vec::new(),
        config,
        byte_vocab,
    })
}

/// 创建通用 tokenizer
pub fn create_tokenizer(tokenizer_type: TokenizerType) -> Result<Box<dyn Tokenizer>> {
    match tokenizer_type {
        TokenizerType::BPE => Ok(Box::new(create_gpt2_tokenizer()?)),
        _ => Err(LLMError::TokenizerError(format!(
            "Unsupported tokenizer type: {:?}",
            tokenizer_type
        ))),
    }
}

/// Tokenize 文本（便捷函数）
pub fn tokenize(text: &str, tokenizer: &dyn Tokenizer) -> Result<TokenizeResult> {
    let input_ids = tokenizer.encode(text)?;

    let attention_mask = Some(vec![1usize; input_ids.len()]);
    let position_ids = Some((0..input_ids.len()).collect());

    Ok(TokenizeResult {
        input_ids,
        attention_mask,
        position_ids,
        token_type_ids: None,
    })
}

/// 批量分词
pub fn batch_tokenize(texts: &[String], tokenizer: &dyn Tokenizer) -> Result<Vec<TokenizeResult>> {
    texts.iter().map(|text| tokenize(text, tokenizer)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpt2_tokenizer() {
        let tokenizer = create_gpt2_tokenizer().unwrap();
        let ids = tokenizer.encode("Hello world").unwrap();
        assert!(!ids.is_empty());
    }
}
