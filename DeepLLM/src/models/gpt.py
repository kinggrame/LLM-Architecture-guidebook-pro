# DeepLLM/src/models/gpt.py
# 
# GPT (Generative Pre-trained Transformer) 模型实现
# 
# 特点：
# - 仅使用解码器 (Decoder-only)
# - 因果注意力 (Causal Attention)
# - 下一个词预测 (Next Token Prediction)

import torch
import torch.nn as nn
import torch.nn.functional as F
import math
from typing import Optional, Tuple


class GPTAttention(nn.Module):
    """
    GPT 专用的注意力机制 (Causal/Autoregressive Attention)
    
    与标准 Transformer 的区别：
    - 只能看到当前位置之前的信息
    - 使用 causal mask
    - 通常使用旋转位置编码 (RoPE)
    """
    
    def __init__(
        self,
        embed_dim: int,
        num_heads: int,
        dropout: float = 0.1,
        bias: bool = True,
        max_seq_len: int = 2048,
    ):
        super().__init__()
        
        assert embed_dim % num_heads == 0
        
        self.embed_dim = embed_dim
        self.num_heads = num_heads
        self.head_dim = embed_dim // num_heads
        self.max_seq_len = max_seq_len
        
        # 投影矩阵 (GPT-2 使用 bias)
        self.qkv_proj = nn.Linear(embed_dim, 3 * embed_dim, bias=bias)
        self.out_proj = nn.Linear(embed_dim, embed_dim, bias=bias)
        
        self.attn_dropout = nn.Dropout(dropout)
        self.resid_dropout = nn.Dropout(dropout)
        
        self.scale = math.sqrt(self.head_dim)
        
        # 注册 causal mask 为 buffer
        self.register_buffer(
            "causal_mask",
            self._create_causal_mask(max_seq_len),
            persistent=False
        )
    
    def _create_causal_mask(self, seq_len: int) -> torch.Tensor:
        """创建因果掩码"""
        # 上三角为 -inf (不可见)，对角线及以下为 0 (可见)
        mask = torch.triu(
            torch.full((seq_len, seq_len), float('-inf'), dtype=torch.float32),
            diagonal=1
        )
        return mask
    
    def forward(
        self,
        x: torch.Tensor,
        attention_mask: Optional[torch.Tensor] = None,
        position_ids: Optional[torch.Tensor] = None,
    ) -> Tuple[torch.Tensor, torch.Tensor]:
        """
        前向传播
        
        参数:
            x: (batch, seq_len, embed_dim)
            attention_mask: 可选的注意力掩码
            position_ids: 位置 IDs (用于 RoPE)
            
        返回:
            output: 注意力输出
            attn_weights: 注意力权重
        """
        batch_size, seq_len, _ = x.size()
        
        # ========== 1. QKV 投影 ==========
        qkv = self.qkv_proj(x)  # (batch, seq_len, 3*embed_dim)
        
        # 分割 Q, K, V
        q, k, v = qkv.chunk(3, dim=-1)
        
        # ========== 2. Reshape 为多头 ==========
        # (batch, seq_len, embed_dim) -> (batch, num_heads, seq_len, head_dim)
        q = q.view(batch_size, seq_len, self.num_heads, self.head_dim).transpose(1, 2)
        k = k.view(batch_size, seq_len, self.num_heads, self.head_dim).transpose(1, 2)
        v = v.view(batch_size, seq_len, self.num_heads, self.head_dim).transpose(1, 2)
        
        # ========== 3. 应用 RoPE (旋转位置编码) ==========
        # 如果有 position_ids，使用 RoPE
        # (简化版本，这里先不做 RoPE)
        
        # ========== 4. 计算注意力分数 ==========
        # (batch, num_heads, seq_len_q, head_dim) @ (batch, num_heads, head_dim, seq_len_k)
        # -> (batch, num_heads, seq_len_q, seq_len_k)
        attn_scores = torch.matmul(q, k.transpose(-2, -1)) / self.scale
        
        # 应用 causal mask
        if seq_len <= self.max_seq_len:
            causal_mask = self.causal_mask[:seq_len, :seq_len]
        else:
            # 动态创建更大的 mask
            causal_mask = self._create_causal_mask(seq_len).to(x.device)
        
        attn_scores = attn_scores + causal_mask
        
        # 应用额外的 attention mask
        if attention_mask is not None:
            attn_scores = attn_scores + attention_mask
        
        # ========== 5. Softmax ==========
        attn_weights = F.softmax(attn_scores, dim=-1)
        attn_weights = self.attn_dropout(attn_weights)
        
        # ========== 6. 加权求和 ==========
        attn_output = torch.matmul(attn_weights, v)
        
        # ========== 7. 合并多头 ==========
        # (batch, num_heads, seq_len, head_dim) -> (batch, seq_len, embed_dim)
        attn_output = attn_output.transpose(1, 2).contiguous()
        attn_output = attn_output.view(batch_size, seq_len, self.embed_dim)
        
        # ========== 8. 输出投影 ==========
        output = self.out_proj(attn_output)
        output = self.resid_dropout(output)
        
        return output, attn_weights


class GPTMLP(nn.Module):
    """
    GPT 的前馈网络 (使用 SwiGLU 激活函数)
    
    SwiGLU = SiLU(xW) * (xV) @ W
    
    比标准的 GELU 效果更好
    """
    
    def __init__(
        self,
        embed_dim: int,
        ff_dim: int = None,
        dropout: float = 0.1,
        bias: bool = True,
    ):
        super().__init__()
        
        if ff_dim is None:
            ff_dim = embed_dim * 4  # GPT-2 默认 4 倍
        
        self.embed_dim = embed_dim
        self.ff_dim = ff_dim
        
        # SwiGLU 使用三个投影
        self.fc1 = nn.Linear(embed_dim, ff_dim, bias=bias)   # 门控
        self.fc2 = nn.Linear(embed_dim, ff_dim, bias=bias)   # 值
        self.fc3 = nn.Linear(ff_dim, embed_dim, bias=bias)   # 输出
        
        self.dropout = nn.Dropout(dropout)
    
    def forward(self, x: torch.Tensor) -> torch.Tensor:
        """SwiGLU: x * sigmoid(x) * W2 @ W3"""
        # SwiGLU 公式
        x_gate = self.fc1(x)
        x_val = self.fc2(x)
        
        # SiLU (Sigmoid Linear Unit) = x * sigmoid(x)
        x_act = F.silu(x_gate)
        
        # 逐元素乘积
        x = x_act * x_val
        
        x = self.fc3(x)
        x = self.dropout(x)
        
        return x


class GPTBlock(nn.Module):
    """
    单层 GPT Transformer Block
    
    包含：
    - GPT Attention (Causal)
    - Add & LayerNorm
    - MLP (SwiGLU)
    - Add & LayerNorm
    """
    
    def __init__(
        self,
        embed_dim: int,
        num_heads: int,
        ff_dim: int = None,
        dropout: float = 0.1,
        layernorm_eps: float = 1e-5,
        bias: bool = True,
        max_seq_len: int = 2048,
    ):
        super().__init__()
        
        # Pre-LayerNorm (更稳定)
        self.ln_1 = nn.LayerNorm(embed_dim, eps=layernorm_eps)
        self.attn = GPTAttention(
            embed_dim, num_heads, dropout, bias, max_seq_len
        )
        
        self.ln_2 = nn.LayerNorm(embed_dim, eps=layernorm_eps)
        self.mlp = GPTMLP(embed_dim, ff_dim, dropout, bias)
    
    def forward(
        self,
        x: torch.Tensor,
        attention_mask: Optional[torch.Tensor] = None,
        position_ids: Optional[torch.Tensor] = None,
    ) -> torch.Tensor:
        # ===== Attention + 残差 =====
        x = x + self.attn(self.ln_1(x), attention_mask, position_ids)[0]
        
        # ===== MLP + 残差 =====
        x = x + self.mlp(self.ln_2(x))
        
        return x


class GPTModel(nn.Module):
    """
    完整的 GPT 模型
    
    结构：
    - Token Embedding
    - 位置编码 (RoPE 或学习式)
    - N 层 Transformer Block
    - LayerNorm
    - 共享的 LM Head
    """
    
    def __init__(
        self,
        vocab_size: int,
        embed_dim: int = 768,
        num_layers: int = 12,
        num_heads: int = 12,
        ff_dim: int = None,
        max_seq_len: int = 1024,
        dropout: float = 0.1,
        layernorm_eps: float = 1e-5,
        bias: bool = True,
    ):
        super().__init__()
        
        self.vocab_size = vocab_size
        self.embed_dim = embed_dim
        self.num_layers = num_layers
        self.num_heads = num_heads
        self.max_seq_len = max_seq_len
        
        # 词嵌入 (GPT-2 权重共享)
        self.wte = nn.Embedding(vocab_size, embed_dim)
        
        # 位置编码 (GPT-2 使用学习式)
        self.wpe = nn.Embedding(max_seq_len, embed_dim)
        
        self.drop = nn.Dropout(dropout)
        
        # Transformer 层
        self.h = nn.ModuleList([
            GPTBlock(
                embed_dim,
                num_heads,
                ff_dim,
                dropout,
                layernorm_eps,
                bias,
                max_seq_len
            )
            for _ in range(num_layers)
        ])
        
        # 最终 LayerNorm
        self.ln_f = nn.LayerNorm(embed_dim, eps=layernorm_eps)
        
        # LM Head (可以与 wte 共享权重)
        self.lm_head = nn.Linear(embed_dim, vocab_size, bias=False)
        
        # 权重共享
        self.lm_head.weight = self.wte.weight
        
        # 初始化
        self._init_weights()
    
    def _init_weights(self):
        """初始化权重"""
        # 投影层使用标准初始化
        for module in self.modules():
            if isinstance(module, nn.Linear):
                nn.init.normal_(module.weight, mean=0.0, std=0.02)
                if module.bias is not None:
                    nn.init.zeros_(module.bias)
            elif isinstance(module, nn.Embedding):
                nn.init.normal_(module.weight, mean=0.0, std=0.02)
    
    def forward(
        self,
        input_ids: torch.Tensor,
        attention_mask: Optional[torch.Tensor] = None,
        position_ids: Optional[torch.Tensor] = None,
        labels: Optional[torch.Tensor] = None,
    ) -> dict:
        """
        前向传播
        
        参数:
            input_ids: (batch_size, seq_len)
            attention_mask: (batch_size, seq_len)
            position_ids: (batch_size, seq_len)
            labels: (batch_size, seq_len) 用于计算 loss
            
        返回:
            dict 包含:
                - logits: (batch_size, seq_len, vocab_size)
                - loss: 交叉熵损失 (如果提供 labels)
        """
        batch_size, seq_len = input_ids.size()
        
        # ===== 1. 词嵌入 + 位置嵌入 =====
        # Token embedding
        token_embeds = self.wte(input_ids)
        
        # Position embedding
        if position_ids is None:
            position_ids = torch.arange(seq_len, device=input_ids.device)
            position_ids = position_ids.unsqueeze(0).expand(batch_size, -1)
        
        pos_embeds = self.wpe(position_ids)
        
        # 合并
        hidden_states = token_embeds + pos_embeds
        hidden_states = self.drop(hidden_states)
        
        # ===== 2. 通过所有 Transformer 层 =====
        for block in self.h:
            hidden_states = block(hidden_states, attention_mask, position_ids)
        
        # ===== 3. 最终 LayerNorm =====
        hidden_states = self.ln_f(hidden_states)
        
        # ===== 4. LM Head =====
        logits = self.lm_head(hidden_states)
        
        # ===== 5. 计算损失 =====
        loss = None
        if labels is not None:
            # Shift for next token prediction
            # 预测下一个词，所以 labels 是 shifted
            shift_logits = logits[..., :-1, :].contiguous()
            shift_labels = labels[..., 1:].contiguous()
            
            # 计算交叉熵
            loss = F.cross_entropy(
                shift_logits.view(-1, shift_logits.size(-1)),
                shift_labels.view(-1),
                ignore_index=-100,  # 忽略 padding
            )
        
        return {
            'logits': logits,
            'loss': loss,
        }
    
    @torch.no_grad()
    def generate(
        self,
        input_ids: torch.Tensor,
        max_new_tokens: int = 100,
        temperature: float = 1.0,
        top_k: int = None,
        top_p: float = None,
        repetition_penalty: float = 1.0,
    ) -> torch.Tensor:
        """
        自回归生成
        
        参数:
            input_ids: 初始序列 (batch_size, cur_len)
            max_new_tokens: 生成的新 token 数量
            temperature: 温度 (越高越随机)
            top_k: Top-k 采样
            top_p: Top-p (Nucleus) 采样
            repetition_penalty: 重复惩罚
        """
        self.eval()
        
        batch_size = input_ids.size(0)
        device = input_ids.device
        cur_len = input_ids.size(1)
        
        # 扩展 attention mask
        attention_mask = torch.ones_like(input_ids)
        
        for _ in range(max_new_tokens):
            # 如果序列太长，截断
            if cur_len >= self.max_seq_len:
                break
            
            # 前向传播
            outputs = self.forward(
                input_ids[:, -self.max_seq_len:],
                attention_mask[:, -self.max_seq_len:],
            )
            
            logits = outputs['logits']  # (batch, seq_len, vocab_size)
            
            # 只取最后一个 token 的 logits
            next_token_logits = logits[:, -1, :]  # (batch, vocab_size)
            
            # 应用 repetition penalty
            if repetition_penalty != 1.0:
                for i in range(batch_size):
                    for token_id in range(self.vocab_size):
                        if next_token_logits[i, token_id] < 0:
                            next_token_logits[i, token_id] /= repetition_penalty
                        else:
                            next_token_logits[i, token_id] *= repetition_penalty
            
            # 应用 temperature
            if temperature != 1.0:
                next_token_logits /= temperature
            
            # Top-k 采样
            if top_k is not None and top_k > 0:
                v, _ = torch.topk(next_token_logits, min(top_k, self.vocab_size))
                next_token_logits[next_token_logits < v[:, [-1]]] = float('-inf')
            
            # Top-p (Nucleus) 采样
            if top_p is not None and top_p < 1.0:
                sorted_logits, sorted_indices = torch.sort(next_token_logits, descending=True)
                cumulative_probs = torch.cumsum(F.softmax(sorted_logits, dim=-1), dim=-1)
                
                # 保留概率和超过 top_p 的 token
                sorted_indices_to_remove = cumulative_probs > top_p
                sorted_indices_to_remove[..., 1:] = sorted_indices_to_remove[..., :-1].clone()
                sorted_indices_to_remove[..., 0] = 0
                
                indices_to_remove = sorted_indices_to_remove.scatter(1, sorted_indices, sorted_indices_to_remove)
                next_token_logits[indices_to_remove] = float('-inf')
            
            # 采样
            probs = F.softmax(next_token_logits, dim=-1)
            next_token = torch.multinomial(probs, num_samples=1)  # (batch, 1)
            
            # 如果是结束 token，跳过
            # (假设 vocab_size-1 是结束 token)
            
            # 追加到序列
            input_ids = torch.cat([input_ids, next_token], dim=1)
            attention_mask = torch.cat([attention_mask, torch.ones(batch_size, 1, device=device)], dim=1)
            cur_len += 1
            
            # 如果所有序列都生成了结束 token，可以提前停止
            # (简化版本，不做提前停止)
        
        return input_ids


def gpt2_small():
    """GPT-2 Small (124M 参数)"""
    return GPTModel(
        vocab_size=50257,
        embed_dim=768,
        num_layers=12,
        num_heads=12,
        ff_dim=768 * 4,
        max_seq_len=1024,
    )


def gpt2_medium():
    """GPT-2 Medium (350M 参数)"""
    return GPTModel(
        vocab_size=50257,
        embed_dim=1024,
        num_layers=24,
        num_heads=16,
        ff_dim=1024 * 4,
        max_seq_len=1024,
    )


def gpt2_large():
    """GPT-2 Large (774M 参数)"""
    return GPTModel(
        vocab_size=50257,
        embed_dim=1280,
        num_layers=36,
        num_heads=20,
        ff_dim=1280 * 4,
        max_seq_len=1024,
    )


if __name__ == "__main__":
    print("=" * 60)
    print("GPT 模型测试")
    print("=" * 60)
    
    # 创建模型
    model = gpt2_small()
    
    # 统计参数量
    total_params = sum(p.numel() for p in model.parameters())
    print(f"模型参数量: {total_params:,} ({total_params/1e6:.1f}M)")
    print(f"模型大小: {total_params * 4 / 1024 / 1024:.1f} MB (FP32)")
    
    # 测试前向传播
    batch_size = 2
    seq_len = 32
    input_ids = torch.randint(0, 50257, (batch_size, seq_len))
    labels = torch.randint(0, 50257, (batch_size, seq_len))
    
    model.eval()
    with torch.no_grad():
        outputs = model(input_ids, labels=labels)
    
    print(f"\n输入: {input_ids.shape}")
    print(f"输出 logits: {outputs['logits'].shape}")
    print(f"Loss: {outputs['loss'].item():.4f}")
    
    # 测试生成
    print("\n测试生成:")
    prompt = torch.randint(0, 50257, (1, 10))
    with torch.no_grad():
        generated = model.generate(prompt, max_new_tokens=20, temperature=0.8, top_k=40)
    
    print(f"生成序列长度: {generated.shape[1]}")
    print(f"生成的前 30 个 token: {generated[0, :30].tolist()}")
    
    print("\n✅ GPT 模型测试通过！")
