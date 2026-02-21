// src/main.rs - MiniLLM 入口文件
//
// 这是一个从零实现的简化版 LLM
// 包含：Tokenizer, Embedding, Attention, Transformer

// 学习项目：允许未完全使用的脚手架代码（dead_code / unused_imports）
#![allow(dead_code, unused_imports)]

mod attention;
mod embedding;
mod model;
mod tokenizer;
mod transformer;
mod utils;

use attention::MultiHeadAttention;
use embedding::{
    Embedding, EmbeddingConfig, PositionEmbedding, PositionEncodingType, TokenEmbedding,
};
use model::GPT;
use tokenizer::{BPETokenizer, Tokenizer};

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║           MiniLLM - 从零学习大语言模型                     ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();

    // 1. 测试分词器
    println!("【1. 分词器测试】");
    let tokenizer = BPETokenizer::char_level(256);
    let text = "Hello, World!";
    let ids = tokenizer.encode(text);
    let decoded = tokenizer.decode(&ids);
    println!("原始文本: {}", text);
    println!("Token IDs: {:?}", ids);
    println!("解码文本: {}", decoded);
    println!("词汇表大小: {}", tokenizer.vocab_size());
    println!();

    // 2. 测试 Token Embedding
    println!("【2. Token 嵌入测试】");
    let token_emb_config = EmbeddingConfig {
        vocab_size: 256,
        embedding_dim: 64,
        max_seq_len: 128,
        trainable: true,
    };
    let token_embedding = TokenEmbedding::new(token_emb_config);
    let input_ids = vec![0, 1, 2, 3, 4];
    let token_emb = token_embedding.forward(&input_ids);
    println!("输入 IDs: {:?}", input_ids);
    println!(
        "Token 嵌入形状: ({}, {})",
        token_emb.shape()[0],
        token_emb.shape()[1]
    );
    println!();

    // 3. 测试位置编码
    println!("【3. 位置编码测试】");
    let pos_embedding = PositionEmbedding::new(PositionEncodingType::Sinusoidal, 64, 128);
    let pos_emb = pos_embedding.forward(&input_ids);
    println!(
        "位置嵌入形状: ({}, {})",
        pos_emb.shape()[0],
        pos_emb.shape()[1]
    );
    println!();

    // 4. 测试注意力机制
    println!("【4. 多头注意力测试】");
    let attention_config = attention::AttentionConfig::new(64, 4);
    let multihead_attn = MultiHeadAttention::new(attention_config);

    // 模拟输入 (batch=1, seq=3, dim=64)
    let x = token_emb.clone() + pos_emb.clone();
    let attn_output = multihead_attn.forward(&x);
    println!(
        "注意力输出形状: ({}, {})",
        attn_output.shape()[0],
        attn_output.shape()[1]
    );
    println!();

    // 5. 完整模型
    println!("【5. GPT 模型测试】");
    let gpt = GPT::new(256, 64, 4, 4, 128);
    println!("GPT 模型参数量: 约 {} 参数", gpt.num_parameters() / 1000);
    println!();

    println!("═══════════════════════════════════════════════════════════");
    println!("  恭喜！MiniLLM 运行成功！");
    println!("  接下来请阅读 docs/ 学习文档深入理解 LLM");
    println!("═══════════════════════════════════════════════════════════");
}
