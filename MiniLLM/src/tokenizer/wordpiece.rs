// src/tokenizer/wordpiece.rs - WordPiece 分词器
//
// WordPiece 算法原理：
// 1. 从基础字符开始
// 2. 贪心地添加能提高语言模型概率的字符
// 3. BERT 使用的分词器

use super::Tokenizer;
use std::collections::HashMap;

/// WordPiece 分词器
pub struct WordPieceTokenizer {
    vocab: HashMap<String, usize>,
    reverse_vocab: HashMap<usize, String>,
    unknown_id: usize,
    max_token_length: usize,
}

impl WordPieceTokenizer {
    pub fn new(vocab: HashMap<String, usize>, max_length: usize) -> Self {
        let reverse_vocab: HashMap<usize, String> =
            vocab.iter().map(|(k, v)| (*v, k.clone())).collect();

        let unknown_id = vocab.get("[UNK]").copied().unwrap_or(0);

        Self {
            vocab,
            reverse_vocab,
            unknown_id,
            max_token_length: max_length,
        }
    }

    /// 编码单个词
    pub fn tokenize_word(&self, word: &str) -> Vec<String> {
        let word = if word.is_empty() {
            return vec![];
        } else {
            word.to_lowercase()
        };

        // 检查词汇表
        if self.vocab.contains_key(&word) {
            return vec![word];
        }

        // 贪心地分割
        let mut tokens = Vec::new();
        let mut start = 0;

        while start < word.len() {
            let mut end = word.len();
            let mut found = false;

            // 从最长的可能开始
            while end > start {
                let substr = &word[start..end];

                if start > 0 {
                    // 添加前缀
                    if self.vocab.contains_key(&format!("##{}", substr)) {
                        tokens.push(format!("##{}", substr));
                        found = true;
                        break;
                    }
                } else if self.vocab.contains_key(substr) {
                    tokens.push(substr.to_string());
                    found = true;
                    break;
                }

                end -= 1;
            }

            if !found {
                // 使用未知 token
                tokens.push("[UNK]".to_string());
                break;
            }

            start = end;
        }

        tokens
    }
}

impl Tokenizer for WordPieceTokenizer {
    fn encode(&self, text: &str) -> Vec<usize> {
        let mut result = vec![];

        // 添加 [CLS]
        if let Some(id) = self.vocab.get("[CLS]") {
            result.push(*id);
        }

        for word in text.split_whitespace() {
            let tokens = self.tokenize_word(word);
            for token in tokens {
                if let Some(id) = self.vocab.get(&token) {
                    result.push(*id);
                } else {
                    result.push(self.unknown_id);
                }
            }
        }

        // 添加 [SEP]
        if let Some(id) = self.vocab.get("[SEP]") {
            result.push(*id);
        }

        result
    }

    fn decode(&self, ids: &[usize]) -> String {
        let mut result = String::new();

        for id in ids {
            if let Some(token) = self.reverse_vocab.get(id) {
                if token.starts_with("##") {
                    result.push_str(&token[2..]);
                } else if token != "[CLS]" && token != "[SEP]" && token != "[PAD]" {
                    if !result.is_empty() && !result.ends_with(' ') {
                        result.push(' ');
                    }
                    result.push_str(token);
                }
            }
        }

        result
    }

    fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    fn unknown_id(&self) -> usize {
        self.unknown_id
    }
}
