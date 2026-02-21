// src/tokenizer/mod.rs - 分词器模块
//
// 本模块实现从零开始的Tokenizer
// 包括：BPE、WordPiece 等算法的简化实现

mod bpe;
mod wordpiece;

pub use bpe::BPETokenizer;
pub use wordpiece::WordPieceTokenizer;

pub trait Tokenizer {
    /// 将文本编码为 token IDs
    fn encode(&self, text: &str) -> Vec<usize>;

    /// 将 token IDs 解码为文本
    fn decode(&self, ids: &[usize]) -> String;

    /// 获取词汇表大小
    fn vocab_size(&self) -> usize;

    /// 获取未知 token 的 ID
    fn unknown_id(&self) -> usize;
}
