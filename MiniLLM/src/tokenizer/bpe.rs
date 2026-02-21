// src/tokenizer/bpe.rs - Byte Pair Encoding (BPE) 分词器
//
// BPE 算法原理：
// 1. 将文本拆分为字符
// 2. 统计相邻字符对的出现频率
// 3. 合并频率最高的字符对
// 4. 重复直到达到目标词汇量
//
// 参考：GPT-2, GPT-3 使用的分词器

use super::Tokenizer;
use std::collections::{HashMap, HashSet};

/// BPE 训练选项
#[derive(Debug, Clone)]
pub struct BPEConfig {
    pub vocab_size: usize,
    pub min_frequency: usize,
    pub special_tokens: Vec<String>,
}

impl Default for BPEConfig {
    fn default() -> Self {
        Self {
            vocab_size: 50000,
            min_frequency: 2,
            special_tokens: vec![
                "<pad>".to_string(),
                "<unk>".to_string(),
                "<bos>".to_string(),
                "<eos>".to_string(),
            ],
        }
    }
}

/// BPE 分词器
pub struct BPETokenizer {
    /// 词汇表：token -> id
    vocab: HashMap<String, usize>,
    /// 反词汇表：id -> token
    reverse_vocab: HashMap<usize, String>,
    /// 合并规则：排名 -> (token1, token2)
    merges: Vec<(String, String)>,
    /// 基础词汇（字符）
    base_vocab: HashSet<String>,
    /// 未知 token ID
    unknown_id: usize,
}

impl BPETokenizer {
    /// 从词汇表创建分词器
    pub fn new(vocab: HashMap<String, usize>, merges: Vec<(String, String)>) -> Self {
        let reverse_vocab: HashMap<usize, String> =
            vocab.iter().map(|(k, v)| (*v, k.clone())).collect();

        let unknown_id = vocab.get("<unk>").copied().unwrap_or(0);

        // 基础词汇是所有单字符
        let base_vocab: HashSet<String> = vocab.keys().filter(|k| k.len() == 1).cloned().collect();

        Self {
            vocab,
            reverse_vocab,
            merges,
            base_vocab,
            unknown_id,
        }
    }

    /// 创建简单的字符级分词器（用于演示）
    pub fn char_level(vocab_size: usize) -> Self {
        let mut vocab = HashMap::new();

        // 添加特殊 token
        vocab.insert("<pad>".to_string(), 0);
        vocab.insert("<unk>".to_string(), 1);
        vocab.insert("<bos>".to_string(), 2);
        vocab.insert("<eos>".to_string(), 3);

        // 添加字符（ASCII 可见字符）
        let mut id = 4;
        for c in 32u8..127u8 {
            let s = (c as char).to_string();
            if !vocab.contains_key(&s) {
                vocab.insert(s, id);
                id += 1;
                if id >= vocab_size {
                    break;
                }
            }
        }

        let reverse_vocab: HashMap<usize, String> =
            vocab.iter().map(|(k, v)| (*v, k.clone())).collect();
        let base_vocab: HashSet<String> = vocab.keys().cloned().collect();

        Self {
            vocab,
            reverse_vocab,
            merges: Vec::new(),
            base_vocab,
            unknown_id: 1,
        }
    }

    /// 对文本进行编码
    pub fn encode_text(&self, text: &str) -> Vec<usize> {
        let mut tokens: Vec<String> = text.chars().map(|c| c.to_string()).collect();

        // 应用 BPE 合并规则
        for (pair, _) in &self.merges {
            let parts: Vec<&str> = pair.split(' ').collect();
            if parts.len() != 2 {
                continue;
            }

            loop {
                let mut i = 0;
                let mut found = false;

                while i < tokens.len().saturating_sub(1) {
                    if tokens[i] == parts[0] && tokens[i + 1] == parts[1] {
                        // 合并
                        tokens[i] = format!("{} {}", parts[0], parts[1]);
                        tokens.remove(i + 1);
                        found = true;
                    } else {
                        i += 1;
                    }
                }

                if !found {
                    break;
                }
            }
        }

        // 转换为 IDs
        tokens
            .iter()
            .map(|t| self.vocab.get(t).copied().unwrap_or(self.unknown_id))
            .collect()
    }

    /// 对 IDs 进行解码
    pub fn decode_ids(&self, ids: &[usize]) -> String {
        ids.iter()
            .map(|id| {
                self.reverse_vocab
                    .get(id)
                    .cloned()
                    .unwrap_or_else(|| "<unk>".to_string())
            })
            .collect()
    }
}

impl Tokenizer for BPETokenizer {
    fn encode(&self, text: &str) -> Vec<usize> {
        self.encode_text(text)
    }

    fn decode(&self, ids: &[usize]) -> String {
        self.decode_ids(ids)
    }

    fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    fn unknown_id(&self) -> usize {
        self.unknown_id
    }
}

/// 演示 BPE 训练（简化版）
pub fn train_bpe_demo(corpus: &[&str], target_vocab: usize) -> BPETokenizer {
    // 统计字符频率
    let mut char_freq: HashMap<String, usize> = HashMap::new();
    for text in corpus {
        for c in text.chars() {
            *char_freq.entry(c.to_string()).or_insert(0) += 1;
        }
    }

    // 创建词汇表
    let mut vocab: HashMap<String, usize> = HashMap::new();
    let mut id = 0;

    // 特殊 token
    for token in ["<pad>", "<unk>", "<bos>", "<eos>"] {
        vocab.insert(token.to_string(), id);
        id += 1;
    }

    // 字符 - 按频率排序
    let mut char_freq_vec: Vec<_> = char_freq.iter().collect();
    char_freq_vec.sort_by(|a, b| b.1.cmp(a.1));

    for (c, _) in char_freq_vec {
        if id >= target_vocab {
            break;
        }
        vocab.insert(c.clone(), id);
        id += 1;
    }

    BPETokenizer::new(vocab, Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_tokenizer() {
        let tokenizer = BPETokenizer::char_level(256);

        let text = "Hello, World!";
        let ids = tokenizer.encode(text);

        println!("Text: {}", text);
        println!("IDs: {:?}", ids);

        let decoded = tokenizer.decode(&ids);
        println!("Decoded: {}", decoded);

        assert_eq!(tokenizer.vocab_size() > 0, true);
    }
}
