# DeepLLM/src/models/transformer.py
# 
# 从零实现 Transformer 架构
# 包含：Multi-Head Attention, Feed-Forward, LayerNorm, 位置编码
#
# 教学目标：
# 1. 理解 Transformer 内部机制
# 2. 掌握 PyTorch 模型构建
# 3. 为后续训练打下基础

import math
import torch
import torch.nn as nn
import torch.nn.functional as F
from typing import Optional, Tuple


class MultiHeadAttention(nn.Module):
    """
    多头注意力机制
    
    原理：
    - 将 Query, Key, Value 投影到多个子空间
    - 每个子空间独立计算注意力
    - 拼接所有头的输出
    
    这是 Transformer 的核心组件
    """
    
    def __init__(
        self, 
        embed_dim: int,           # 嵌入维度
        num_heads: int,           # 注意力头数
        dropout: float = 0.1,    # Dropout 概率
        bias: bool = True         # 是否有偏置
    ):
        super().__init__()
        
        # 参数验证
        assert embed_dim % num_heads == 0, "embed_dim 必须能被 num_heads 整除"
        
        self.embed_dim = embed_dim
        self.num_heads = num_heads
        self.head_dim = embed_dim // num_heads  # 每个头的维度
        self.dropout = dropout
        
        # 投影矩阵 (可以合并为一个线性层以提高效率)
        # 原始实现：四个独立的矩阵
        self.q_proj = nn.Linear(embed_dim, embed_dim, bias=bias)
        self.k_proj = nn.Linear(embed_dim, embed_dim, bias=bias)
        self.v_proj = nn.Linear(embed_dim, embed_dim, bias=bias)
        self.out_proj = nn.Linear(embed_dim, embed_dim, bias=bias)
        
        # Dropout
        self.attn_dropout = nn.Dropout(dropout)
        
        # 缩放因子 (√d_k)
        self.scale = math.sqrt(self.head_dim)
    
    def forward(
        self,
        query: torch.Tensor,      # (batch, seq_len_q, embed_dim)
        key: torch.Tensor,        # (batch, seq_len_k, embed_dim)
        value: torch.Tensor,     # (batch, seq_len_v, embed_dim)
        attn_mask: Optional[torch.Tensor] = None,
        key_padding_mask: Optional[torch.Tensor] = None,
    ) -> Tuple[torch.Tensor, torch.Tensor]:
        """
        前向传播
        
        参数:
            query: 查询向量
            key: 键向量  
            value: 值向量
            attn_mask: 注意力掩码 (用于控制哪些位置可以 attend)
            key_padding_mask: 键的填充掩码 (用于 padding 的位置)
            
        返回:
            output: 注意力输出
            attn_weights: 注意力权重 (用于可视化)
        """
        batch_size = query.size(0)
        
        # ========== 1. 线性投影 ==========
        # Q, K, V 分别投影到 num_heads 个子空间
        Q = self.q_proj(query)   # (batch, seq_len, embed_dim)
        K = self.k_proj(key)
        V = self.v_proj(value)
        
        # ========== 2. 分割成多个头 ==========
        # 形状: (batch, num_heads, seq_len, head_dim)
        Q = Q.view(batch_size, -1, self.num_heads, self.head_dim).transpose(1, 2)
        K = K.view(batch_size, -1, self.num_heads, self.head_dim).transpose(1, 2)
        V = V.view(batch_size, -1, self.num_heads, self.head_dim).transpose(1, 2)
        
        # ========== 3. 计算注意力分数 ==========
        # Q @ K^T / √d_k
        # 形状: (batch, num_heads, seq_len_q, seq_len_k)
        attn_scores = torch.matmul(Q, K.transpose(-2, -1)) / self.scale
        
        # 应用掩码 (如果提供)
        if attn_mask is not None:
            attn_scores = attn_scores.masked_fill(attn_mask == 0, float('-inf'))
        
        # Padding 掩码
        if key_padding_mask is not None:
            # 扩展为 (batch, 1, 1, seq_len_k)
            key_padding_mask = key_padding_mask.unsqueeze(1).unsqueeze(2)
            attn_scores = attn_scores.masked_fill(key_padding_mask == 0, float('-inf'))
        
        # ========== 4. Softmax ==========
        attn_weights = F.softmax(attn_scores, dim=-1)
        attn_weights = self.attn_dropout(attn_weights)
        
        # ========== 5. 注意力输出 ==========
        # attn_weights @ V
        # 形状: (batch, num_heads, seq_len, head_dim)
        attn_output = torch.matmul(attn_weights, V)
        
        # ========== 6. 合并多头输出 ==========
        # (batch, num_heads, seq_len, head_dim) -> (batch, seq_len, embed_dim)
        attn_output = attn_output.transpose(1, 2).contiguous()
        attn_output = attn_output.view(batch_size, -1, self.embed_dim)
        
        # 最终线性投影
        output = self.out_proj(attn_output)
        
        return output, attn_weights


class FeedForward(nn.Module):
    """
    前馈神经网络 (Feed-Forward Network)
    
    公式: FFN(x) = GELU(xW1 + b1)W2 + b2
    
    通常 hidden_dim = 4 * embed_dim
    """
    
    def __init__(
        self, 
        embed_dim: int,
        ff_dim: int = None,       # 隐藏层维度，默认 4*embed_dim
        dropout: float = 0.1,
        activation: str = "gelu"   # 激活函数: gelu 或 relu
    ):
        super().__init__()
        
        if ff_dim is None:
            ff_dim = embed_dim * 4
            
        self.embed_dim = embed_dim
        self.ff_dim = ff_dim
        
        # 两层线性变换
        self.w1 = nn.Linear(embed_dim, ff_dim, bias=True)
        self.w2 = nn.Linear(ff_dim, embed_dim, bias=True)
        self.dropout = nn.Dropout(dropout)
        
        # 激活函数
        if activation == "gelu":
            self.activation = nn.GELU()
        elif activation == "relu":
            self.activation = nn.ReLU()
        else:
            raise ValueError(f"Unknown activation: {activation}")
    
    def forward(self, x: torch.Tensor) -> torch.Tensor:
        """
        前向传播
        
        x: (batch, seq_len, embed_dim)
        -> (batch, seq_len, ff_dim)
        -> (batch, seq_len, embed_dim)
        """
        x = self.w1(x)
        x = self.activation(x)
        x = self.dropout(x)
        x = self.w2(x)
        x = self.dropout(x)
        return x


class PositionalEncoding(nn.Module):
    """
    位置编码 (Positional Encoding)
    
    使用正弦和余弦函数编码位置信息
    
    PE(pos, 2i)   = sin(pos / 10000^(2i/d))
    PE(pos, 2i+1) = cos(pos / 10000^(2i/d))
    
    优点：
    - 能够表示相对位置
    - 外推到更长序列
    """
    
    def __init__(self, embed_dim: int, max_len: int = 5000, dropout: float = 0.1):
        super().__init__()
        self.dropout = nn.Dropout(p=dropout)
        
        # 创建位置编码矩阵
        pe = torch.zeros(max_len, embed_dim)
        position = torch.arange(0, max_len, dtype=torch.float).unsqueeze(1)
        
        # 计算除数项
        div_term = torch.exp(
            torch.arange(0, embed_dim, 2, dtype=torch.float) * 
            (-math.log(10000.0) / embed_dim)
        )
        
        # 偶数位置用 sin
        pe[:, 0::2] = torch.sin(position * div_term)
        # 奇数位置用 cos
        pe[:, 1::2] = torch.cos(position * div_term)
        
        # 添加 batch 维度并注册为 buffer (不参与训练)
        pe = pe.unsqueeze(0)  # (1, max_len, embed_dim)
        self.register_buffer('pe', pe)
    
    def forward(self, x: torch.Tensor) -> torch.Tensor:
        """
        添加位置编码
        
        x: (batch, seq_len, embed_dim)
        """
        seq_len = x.size(1)
        # 切片获取对应位置的位置编码
        x = x + self.pe[:, :seq_len, :]
        return self.dropout(x)


class TransformerLayer(nn.Module):
    """
    单层 Transformer
    
    包含：
    1. Multi-Head Attention (带掩码)
    2. Add & Norm (残差连接 + LayerNorm)
    3. Feed-Forward Network
    4. Add & Norm
    """
    
    def __init__(
        self,
        embed_dim: int,
        num_heads: int,
        ff_dim: int = None,
        dropout: float = 0.1,
        attn_dropout: float = 0.1,
    ):
        super().__init__()
        
        # Multi-Head Attention
        self.self_attn = MultiHeadAttention(
            embed_dim, 
            num_heads, 
            dropout=attn_dropout
        )
        
        # Feed-Forward Network
        self.ffn = FeedForward(embed_dim, ff_dim, dropout)
        
        # Layer Norm
        self.norm1 = nn.LayerNorm(embed_dim)
        self.norm2 = nn.LayerNorm(embed_dim)
        
        # Dropout
        self.dropout = nn.Dropout(dropout)
    
    def forward(
        self,
        x: torch.Tensor,
        attn_mask: Optional[torch.Tensor] = None,
        key_padding_mask: Optional[torch.Tensor] = None,
    ) -> torch.Tensor:
        """
        前向传播
        
        残差连接顺序：先 LayerNorm，再Attention/FFN
        (Pre-Layer Normalization，比 Post-LN 更稳定)
        """
        # ===== 第一个子层: Self-Attention + 残差 =====
        # Pre-LN: 先 LayerNorm
        x_norm = self.norm1(x)
        
        # Self-Attention
        attn_output, _ = self.self_attn(
            x_norm, x_norm, x_norm,
            attn_mask=attn_mask,
            key_padding_mask=key_padding_mask
        )
        
        # 残差连接 + Dropout
        x = x + self.dropout(attn_output)
        
        # ===== 第二个子层: FFN + 残差 =====
        # Pre-LN
        x_norm = self.norm2(x)
        
        # FFN
        ffn_output = self.ffn(x_norm)
        
        # 残差连接
        x = x + ffn_output
        
        return x


class Transformer(nn.Module):
    """
    完整 Transformer 模型
    
    编码器 (Encoder) 结构
    """
    
    def __init__(
        self,
        vocab_size: int,              # 词汇表大小
        embed_dim: int = 512,         # 嵌入维度
        num_layers: int = 6,         # 层数
        num_heads: int = 8,           # 注意力头数
        ff_dim: int = None,           # FFN 隐藏层维度
        dropout: float = 0.1,
        max_len: int = 5000,          # 最大序列长度
    ):
        super().__init__()
        
        self.vocab_size = vocab_size
        self.embed_dim = embed_dim
        self.num_layers = num_layers
        
        # 词嵌入
        self.token_embedding = nn.Embedding(vocab_size, embed_dim)
        self.embedding_dropout = nn.Dropout(dropout)
        
        # 位置编码
        self.pos_encoding = PositionalEncoding(embed_dim, max_len, dropout)
        
        # Transformer 层堆叠
        self.layers = nn.ModuleList([
            TransformerLayer(
                embed_dim,
                num_heads,
                ff_dim,
                dropout
            )
            for _ in range(num_layers)
        ])
        
        # 输出 Layer Norm
        self.final_norm = nn.LayerNorm(embed_dim)
        
        # 权重初始化
        self._init_weights()
    
    def _init_weights(self):
        """权重初始化 - 使用标准差 0.02 的正态分布"""
        for p in self.parameters():
            if p.dim() > 1:
                nn.init.normal_(p, mean=0.0, std=0.02)
    
    def forward(
        self,
        input_ids: torch.Tensor,           # (batch, seq_len)
        attention_mask: Optional[torch.Tensor] = None,  # (batch, seq_len)
    ) -> torch.Tensor:
        """
        前向传播
        
        参数:
            input_ids: 输入 token IDs
            attention_mask: 注意力掩码
            
        返回:
            logits: (batch, seq_len, vocab_size)
        """
        batch_size, seq_len = input_ids.size()
        
        # ===== 1. 词嵌入 + 位置编码 =====
        # (batch, seq_len) -> (batch, seq_len, embed_dim)
        x = self.token_embedding(input_ids) * math.sqrt(self.embed_dim)
        x = self.embedding_dropout(x)
        x = self.pos_encoding(x)
        
        # ===== 2. 准备掩码 =====
        # 创建 padding mask (位置为 0 的地方掩码)
        key_padding_mask = (input_ids != 0)  # (batch, seq_len)
        
        # 创建因果掩码 (用于解码器，防止看到未来信息)
        causal_mask = self._generate_causal_mask(seq_len, input_ids.device)
        
        # ===== 3. 通过所有 Transformer 层 =====
        for layer in self.layers:
            x = layer(
                x,
                attn_mask=causal_mask,
                key_padding_mask=key_padding_mask
            )
        
        # ===== 4. 最终 Layer Norm =====
        x = self.final_norm(x)
        
        # ===== 5. 输出 logits =====
        # 共享权重：output = x @ embedding^T
        logits = F.linear(x, self.token_embedding.weight)
        
        return logits
    
    def _generate_causal_mask(self, seq_len: int, device: torch.device) -> torch.Tensor:
        """
        生成因果掩码
        
        确保位置 i 只能 attend 到 0 到 i 的位置
        """
        # 上三角矩阵 (对角线及以下为 True)
        mask = torch.triu(
            torch.ones(seq_len, seq_len, device=device, dtype=torch.bool),
            diagonal=1
        )
        return mask
    
    def generate_square_subsequent_mask(self, sz: int) -> torch.Tensor:
        """生成解码器用的掩码"""
        mask = (torch.triu(torch.ones(sz, sz)) == 1).transpose(0, 1)
        mask = mask.float().masked_fill(mask == 0, float('-inf')).masked_fill(mask == 1, float(0.0))
        return mask


def count_parameters(model: nn.Module) -> int:
    """统计模型参数量"""
    return sum(p.numel() for p in model.parameters() if p.requires_grad)


if __name__ == "__main__":
    # 测试代码
    print("=" * 50)
    print("测试 Transformer 模型")
    print("=" * 50)
    
    # 创建模型
    model = Transformer(
        vocab_size=30000,
        embed_dim=512,
        num_layers=6,
        num_heads=8,
    )
    
    # 参数量
    total_params = count_parameters(model)
    print(f"模型参数量: {total_params:,}")
    print(f"模型大小: {total_params * 4 / 1024 / 1024:.2f} MB (FP32)")
    
    # 测试前向传播
    batch_size = 2
    seq_len = 32
    input_ids = torch.randint(0, 30000, (batch_size, seq_len))
    
    model.eval()
    with torch.no_grad():
        logits = model(input_ids)
    
    print(f"\n输入形状: {input_ids.shape}")
    print(f"输出形状: {logits.shape}")
    
    print("\n✅ Transformer 模型测试通过！")
