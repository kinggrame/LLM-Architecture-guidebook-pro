# MiniLLM - 从零学习大语言模型

> 🎯 目标：从 LLM 小白到可以应聘 AI 算法工程师

---

## 项目概述

**MiniLLM** 是一个从零实现的简化版大语言模型（LLM），使用 Rust 编写。

### 项目架构

```
MiniLLM/
├── Cargo.toml
├── src/
│   ├── main.rs              # 入口
│   ├── tokenizer/           # 分词器
│   │   ├── mod.rs
│   │   ├── bpe.rs          # BPE 分词
│   │   └── wordpiece.rs    # WordPiece 分词
│   ├── embedding/           # 词嵌入
│   │   ├── mod.rs
│   │   ├── token_embedding.rs
│   │   └── position_embedding.rs
│   ├── attention/           # 注意力机制
│   │   ├── mod.rs
│   │   └── multi_head_attention.rs
│   ├── transformer/         # Transformer
│   │   ├── mod.rs
│   │   ├── transformer_layer.rs
│   │   └── gpt.rs
│   ├── model/              # 模型定义
│   └── utils/             # 工具函数
└── docs/                   # 学习文档
```

---

## 学习路线（30天）

### 第一阶段：基础（Day 1-10）

| 天数 | 主题 | 代码模块 | 关键词 |
|:----:|------|----------|--------|
| Day 1 | LLM 基础概念 | - | Transformer, GPT, Attention |
| Day 2 | 分词器原理 | tokenizer/ | BPE, WordPiece, Token |
| Day 3 | 词嵌入 | embedding/ | Word2Vec, Embedding, Vector |
| Day 4 | 位置编码 | position_embedding.rs | Sinusoidal, Positional Encoding |
| Day 5 | 注意力机制 | attention/ | Q, K, V, Attention Score |
| Day 6 | 多头注意力 | multi_head_attention.rs | Multi-Head, Head Dim |
| Day 7 | Transformer 层 | transformer_layer.rs | Add & Norm, FFN, Residual |
| Day 8 | GPT 模型 | gpt.rs | LM Head, Causal Mask, Next Token |
| Day 9 | 前向传播 | gpt.rs | Forward Pass, Logits |
| Day 10 | 文本生成 | gpt.rs | Sampling, Greedy, Top-k |

### 第二阶段：进阶（Day 11-20）

| 天数 | 主题 | 关键词 |
|:----:|------|--------|
| Day 11 | 反向传播 | Backpropagation, Gradient, Chain Rule |
| Day 12 | 损失函数 | Cross-Entropy, NLL Loss |
| Day 13 | 优化器 | Adam, SGD, Learning Rate |
| Day 14 | 训练循环 | Training Loop, Epoch, Batch |
| Day 15 | 模型量化 | Quantization, INT8, Pruning |
| Day 16 | 推理优化 | Inference, KV Cache, Batch Inference |
| Day 17 | 分布式训练 | Data Parallel, Model Parallel, ZeRO |
| Day 18 | 微调技术 | Fine-tuning, LoRA, PEFT |
| Day 19 | RLHF | Reinforcement Learning, PPO, Reward Model |
| Day 20 | 评估指标 | Perplexity, BLEU, ROUGE |

### 第三阶段：实践（Day 21-30）

| 天数 | 项目 |
|:----:|------|
| Day 21-22 | 实现自己的 tokenizer |
| Day 23-24 | 实现 MiniGPT |
| Day 25-26 | 使用 GGML 加载模型 |
| Day 27-28 | 实现量化推理 |
| Day 29-30 | 部署到端侧 |

---

## 核心概念详解

### 1. 分词器 (Tokenizer)

**为什么需要分词？**
- 计算机无法直接处理文本
- 需要将文本转换为数字

**常见方法：**
- 字符级（Character-level）
- 词级（Word-level）
- 子词级（Subword-level）：BPE, WordPiece

**代码位置**：`src/tokenizer/bpe.rs`

### 2. 词嵌入 (Embedding)

**核心思想：**
- 将离散符号映射到连续向量空间
- 语义相似的词在向量空间中距离近

**类型：**
- Token Embedding
- Position Embedding

**代码位置**：`src/embedding/`

### 3. 注意力机制 (Attention)

**核心公式：**
```
Attention(Q, K, V) = softmax(QK^T / √d_k) V
```

**关键概念：**
- Query (Q): 我想查询什么
- Key (K): 我包含什么信息
- Value (V): 匹配成功后返回什么

**代码位置**：`src/attention/`

### 4. Transformer

**架构组成：**
1. Multi-Head Attention
2. Add & LayerNorm
3. Feed Forward Network
4. Add & LayerNorm

**代码位置**：`src/transformer/`

### 5. GPT

**特点：**
- 只使用 Decoder 部分
- 因果注意力（Causal Mask）
- 下一个词预测

**代码位置**：`src/transformer/gpt.rs`

---

## 关键代码片段

### 注意力计算

```rust
// src/attention/multi_head_attention.rs

/// 计算注意力分数
fn compute_attention_scores(
    query: &Array2<f32>,
    key: &Array2<f32>,
) -> Array2<f32> {
    // 1. QK^T 矩阵乘法
    let scores = query.dot(&key.t());
    
    // 2. 缩放（防止梯度消失）
    let head_dim = query.shape()[1];
    let scale = (head_dim as f32).sqrt();
    let scaled = scores / scale;
    
    // 3. Softmax
    softmax(&scaled)
}
```

### GPT 前向传播

```rust
// src/transformer/gpt.rs

pub fn forward(&self, input_ids: &[usize]) -> Vec<Vec<f32>> {
    // 1. Token + Position Embedding
    let token_emb = self.token_embedding.forward(input_ids);
    let position_emb = self.position_embedding.forward(&position_ids);
    let hidden = token_emb + position_emb;
    
    // 2. 通过 Transformer 层
    for layer in &self.layers {
        hidden = layer.forward(&hidden);
    }
    
    // 3. LM Head（语言模型头）
    let logits = hidden.dot(&self.lm_head.t());
    
    // 4. Softmax
    softmax(&logits)
}
```

---

## 面试常见问题

### 1. Transformer vs LSTM

| 方面 | Transformer | LSTM |
|------|-------------|------|
| 注意力 | 全局 | 局部 |
| 并行化 | 容易 | 困难 |
| 长距离依赖 | 容易 | 困难 |
| 计算量 | O(n²·d) | O(n·d²) |

### 2. 为什么需要位置编码？

因为 Attention 本身是位置无关的，需要额外添加位置信息。

### 3. GPT vs BERT

| 方面 | GPT | BERT |
|------|-----|------|
| 架构 | Decoder | Encoder |
| 训练目标 | 下一个词预测 | 掩码语言模型 |
| 适用场景 | 生成 | 理解 |

### 4. 注意力复杂度

- 空间复杂度：O(n²)
- 时间复杂度：O(n²·d)

### 5. 量化技术

- INT8, INT4
- 动态量化 vs 静态量化
- GPTQ, AWQ

---

## 推荐学习资源

### 论文

1. **Attention Is All You Need** (2017) - Transformer 原始论文
2. **GPT** (2018) - 语言模型
3. **GPT-2** (2019) - 多任务学习
4. **GPT-3** (2020) - 零样本学习
5. **Llama** (2023) - 开源大模型

### 课程

1. **CS224N** - Stanford NLP with Deep Learning
2. **Fast.ai** - Practical Deep Learning
3. **Hugging Face** - NLP Course

### 书籍

1. 《深度学习入门：基于 Python 的理论与实现》
2. 《动手学深度学习》
3. 《Pattern Recognition and Machine Learning》

---

## 就业方向

### AI 算法工程师

**技能要求：**
- 扎实的机器学习基础
- 深度学习框架（PyTorch/TensorFlow）
- Transformer 架构
- 大模型训练/微调经验
- 分布式训练

**岗位方向：**
- 大模型预训练
- 垂直领域微调
- AI 应用开发
- 模型优化

---

## 下一步

1. **运行项目**：
   ```bash
   cd MiniLLM
   cargo run
   ```

2. **阅读源码**：按照模块顺序学习

3. **动手实践**：尝试修改模型参数

---

*持续更新中...*
