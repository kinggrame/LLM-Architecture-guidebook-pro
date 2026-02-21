# DeepLLM/src/training/trainer.py
# 
# 训练器核心模块
# 包含：数据加载、训练循环、优化器、学习率调度

import os
import time
import torch
import torch.nn as nn
from torch.utils.data import Dataset, DataLoader
from torch.optim import AdamW, Adam
from torch.optim.lr_scheduler import CosineAnnealingLR, LambdaLR
from typing import Optional, Dict, Any, Callable
from tqdm import tqdm
import numpy as np


class TextDataset(Dataset):
    """
    文本数据集
    
    将原始文本转换为 token IDs 序列
    """
    
    def __init__(
        self,
        token_ids: list,       # 已经 token 化 的 IDs
        block_size: int = 1024,  # 训练使用的序列长度
    ):
        self.token_ids = token_ids
        self.block_size = block_size
    
    def __len__(self):
        # 每个序列需要一个 block_size 的上下文
        return max(0, len(self.token_ids) - self.block_size)
    
    def __getitem__(self, idx: int) -> Dict[str, torch.Tensor]:
        """
        返回:
            input_ids: (block_size,) 输入序列
            labels: (block_size,) 目标序列 (shifted)
        """
        # 从 token_ids 中截取 block_size + 1 个 token
        # 用于生成 input 和 label (shifted by 1)
        start = idx
        end = start + self.block_size + 1
        
        tokens = self.token_ids[start:end]
        
        input_ids = tokens[:-1]  # (block_size,)
        labels = tokens[1:]      # (block_size,) 目标
        
        return {
            'input_ids': torch.tensor(input_ids, dtype=torch.long),
            'labels': torch.tensor(labels, dtype=torch.long),
        }


class TrainingConfig:
    """训练配置"""
    
    def __init__(
        self,
        # 模型
        vocab_size: int = 50257,
        embed_dim: int = 768,
        num_layers: int = 12,
        num_heads: int = 12,
        max_seq_len: int = 1024,
        
        # 数据
        block_size: int = 1024,
        
        # 训练
        batch_size: int = 8,
        learning_rate: float = 3e-4,
        num_epochs: int = 3,
        weight_decay: float = 0.01,
        beta1: float = 0.9,
        beta2: float = 0.95,
        eps: float = 1e-8,
        
        # 梯度
        gradient_clip: float = 1.0,
        accumulation_steps: int = 1,
        
        # 学习率调度
        warmup_steps: int = 500,
        min_lr: float = 1e-5,
        
        # 设备
        device: str = "cuda" if torch.cuda.is_available() else "cpu",
        
        # 其他
        seed: int = 42,
        save_dir: str = "./checkpoints",
        log_interval: int = 10,
        eval_interval: int = 1000,
    ):
        self.vocab_size = vocab_size
        self.embed_dim = embed_dim
        self.num_layers = num_layers
        self.num_heads = num_heads
        self.max_seq_len = max_seq_len
        self.block_size = block_size
        self.batch_size = batch_size
        self.learning_rate = learning_rate
        self.num_epochs = num_epochs
        self.weight_decay = weight_decay
        self.beta1 = beta1
        self.beta2 = beta2
        self.eps = eps
        self.gradient_clip = gradient_clip
        self.accumulation_steps = accumulation_steps
        self.warmup_steps = warmup_steps
        self.min_lr = min_lr
        self.device = device
        self.seed = seed
        self.save_dir = save_dir
        self.log_interval = log_interval
        self.eval_interval = eval_interval
    
    def __repr__(self):
        return f"TrainingConfig({self.__dict__})"


class Trainer:
    """
    训练器
    
    负责：
    - 模型训练
    - 优化器管理
    - 学习率调度
    - Checkpoint 保存/加载
    - 日志记录
    """
    
    def __init__(
        self,
        model: nn.Module,
        config: TrainingConfig,
        train_loader: DataLoader,
        eval_loader: Optional[DataLoader] = None,
    ):
        self.model = model
        self.config = config
        self.train_loader = train_loader
        self.eval_loader = eval_loader
        
        # 设备
        self.device = torch.device(config.device)
        self.model.to(self.device)
        
        # 优化器
        self.optimizer = self._create_optimizer()
        
        # 学习率调度器
        self.scheduler = self._create_scheduler()
        
        # 训练状态
        self.global_step = 0
        self.current_epoch = 0
        self.best_eval_loss = float('inf')
        
        # 随机种子
        self._set_seed(config.seed)
        
        # 创建保存目录
        os.makedirs(config.save_dir, exist_ok=True)
    
    def _set_seed(self, seed: int):
        """设置随机种子"""
        torch.manual_seed(seed)
        torch.cuda.manual_seed_all(seed)
        np.random.seed(seed)
    
    def _create_optimizer(self) -> torch.optim.Optimizer:
        """创建优化器"""
        # 分层学习率：embedding 层使用较小学习率
        no_decay = ["bias", "LayerNorm.weight", "layernorm.weight"]
        
        optimizer_grouped_parameters = [
            {
                "params": [p for n, p in self.model.named_parameters() 
                          if not any(nd in n for nd in no_decay)],
                "weight_decay": self.config.weight_decay,
            },
            {
                "params": [p for n, p in self.model.named_parameters() 
                          if any(nd in n for nd in no_decay)],
                "weight_decay": 0.0,
            },
        ]
        
        return AdamW(
            optimizer_grouped_parameters,
            lr=self.config.learning_rate,
            betas=(self.config.beta1, self.config.beta2),
            eps=self.config.eps,
        )
    
    def _create_scheduler(self):
        """创建学习率调度器"""
        # Warmup + Cosine Annealing
        def lr_lambda(step: int):
            if step < self.config.warmup_steps:
                # Linear warmup
                return step / max(1, self.config.warmup_steps)
            else:
                # Cosine decay
                progress = (step - self.config.warmup_steps) / max(
                    1, self.config.num_epochs * len(self.train_loader) - self.config.warmup_steps
                )
                return max(self.config.min_lr / self.config.learning_rate, 
                         0.5 * (1.0 + np.cos(np.pi * progress)))
        
        return LambdaLR(self.optimizer, lr_lambda)
    
    def train(self) -> Dict[str, list]:
        """
        执行完整训练
        
        返回:
            training_history: 包含训练过程的日志
        """
        print("=" * 60)
        print("开始训练")
        print(f"设备: {self.device}")
        print(f"训练样本数: {len(self.train_loader.dataset)}")
        print(f"训练步数/轮: {len(self.train_loader)}")
        print(f"总步数: {len(self.train_loader) * self.config.num_epochs}")
        print("=" * 60)
        
        history = {
            'train_loss': [],
            'eval_loss': [],
            'learning_rate': [],
            'epoch_time': [],
        }
        
        for epoch in range(self.config.num_epochs):
            self.current_epoch = epoch
            epoch_start_time = time.time()
            
            # 训练一轮
            train_loss = self.train_epoch()
            history['train_loss'].extend(train_loss)
            
            # 评估
            if self.eval_loader is not None:
                eval_loss = self.evaluate()
                history['eval_loss'].append(eval_loss)
                
                # 保存 best model
                if eval_loss < self.best_eval_loss:
                    self.best_eval_loss = eval_loss
                    self.save_checkpoint("best.pt")
                    print(f"✅ 保存最佳模型 (eval_loss: {eval_loss:.4f})")
            
            # 记录学习率
            current_lr = self.optimizer.param_groups[0]['lr']
            history['learning_rate'].append(current_lr)
            
            epoch_time = time.time() - epoch_start_time
            history['epoch_time'].append(epoch_time)
            
            # 保存 checkpoint
            self.save_checkpoint(f"epoch_{epoch}.pt")
            
            print(f"\nEpoch {epoch+1}/{self.config.num_epochs} 完成")
            print(f"  训练 Loss: {np.mean(train_loss[-100:]):.4f}")
            print(f"  学习率: {current_lr:.2e}")
            print(f"  用时: {epoch_time:.1f}s")
            print("-" * 40)
        
        print("\n✅ 训练完成！")
        return history
    
    def train_epoch(self) -> list:
        """训练一轮"""
        self.model.train()
        
        losses = []
        progress_bar = tqdm(self.train_loader, desc=f"Epoch {self.current_epoch+1}")
        
        for batch_idx, batch in enumerate(progress_bar):
            # 移动到设备
            input_ids = batch['input_ids'].to(self.device)
            labels = batch['labels'].to(self.device)
            
            # 前向传播
            outputs = self.model(input_ids, labels=labels)
            loss = outputs['loss']
            
            # 梯度累积
            if self.config.accumulation_steps > 1:
                loss = loss / self.config.accumulation_steps
            
            # 反向传播
            loss.backward()
            
            # 梯度裁剪
            if self.config.gradient_clip > 0:
                torch.nn.utils.clip_grad_norm_(
                    self.model.parameters(), 
                    self.config.gradient_clip
                )
            
            # 更新参数
            if (batch_idx + 1) % self.config.accumulation_steps == 0:
                self.optimizer.step()
                self.scheduler.step()
                self.optimizer.zero_grad()
                self.global_step += 1
            
            # 记录
            losses.append(loss.item() * self.config.accumulation_steps)
            
            # 更新进度条
            progress_bar.set_postfix({
                'loss': f"{np.mean(losses[-10:]):.4f}",
                'lr': f"{self.optimizer.param_groups[0]['lr']:.2e}",
            })
        
        # 最后一批参数更新
        if (batch_idx + 1) % self.config.accumulation_steps != 0:
            self.optimizer.step()
            self.optimizer.zero_grad()
        
        return losses
    
    @torch.no_grad()
    def evaluate(self) -> float:
        """评估模型"""
        self.model.eval()
        
        total_loss = 0
        num_batches = 0
        
        for batch in tqdm(self.eval_loader, desc="Evaluating"):
            input_ids = batch['input_ids'].to(self.device)
            labels = batch['labels'].to(self.device)
            
            outputs = self.model(input_ids, labels=labels)
            loss']
            
            total_loss += loss.item = outputs['loss()
            num_batches += 1
        
        avg_loss = total_loss / num_batches
        perplexity = np.exp(avg_loss)
        
        print(f"\n评估结果:")
        print(f"  Loss: {avg_loss:.4f}")
        print(f"  Perplexity: {perplexity:.2f}")
        
        self.model.train()
        return avg_loss
    
    def save_checkpoint(self, filename: str):
        """保存 checkpoint"""
        checkpoint = {
            'epoch': self.current_epoch,
            'global_step': self.global_step,
            'model_state_dict': self.model.state_dict(),
            'optimizer_state_dict': self.optimizer.state_dict(),
            'scheduler_state_dict': self.scheduler.state_dict(),
            'best_eval_loss': self.best_eval_loss,
            'config': self.config.__dict__,
        }
        
        path = os.path.join(self.config.save_dir, filename)
        torch.save(checkpoint, path)
    
    def load_checkpoint(self, filename: str):
        """加载 checkpoint"""
        path = os.path.join(self.config.save_dir, filename)
        
        if not os.path.exists(path):
            raise FileNotFoundError(f"Checkpoint not found: {path}")
        
        checkpoint = torch.load(path, map_location=self.device)
        
        self.model.load_state_dict(checkpoint['model_state_dict'])
        self.optimizer.load_state_dict(checkpoint['optimizer_state_dict'])
        self.scheduler.load_state_dict(checkpoint['scheduler_state_dict'])
        self.current_epoch = checkpoint['epoch']
        self.global_step = checkpoint['global_step']
        self.best_eval_loss = checkpoint['best_eval_loss']
        
        print(f"✅ 加载 checkpoint: {filename}")
        print(f"  Epoch: {self.current_epoch}")
        print(f"  Global Step: {self.global_step}")
        print(f"  Best Eval Loss: {self.best_eval_loss:.4f}")


def count_trainable_parameters(model: nn.Module) -> int:
    """统计可训练参数数量"""
    return sum(p.numel() for p in model.parameters() if p.requires_grad)


if __name__ == "__main__":
    # 测试代码
    print("=" * 50)
    print("训练器测试")
    print("=" * 50)
    
    from src.models.gpt import gpt2_small
    
    # 创建模型
    model = gpt2_small()
    print(f"模型参数量: {count_trainable_parameters(model):,}")
    
    # 模拟数据
    fake_data = list(range(10000))
    train_dataset = TextDataset(fake_data, block_size=128)
    train_loader = DataLoader(train_dataset, batch_size=4, shuffle=True)
    
    # 配置
    config = TrainingConfig(
        batch_size=4,
        num_epochs=1,
        learning_rate=1e-4,
        log_interval=5,
    )
    
    # 创建训练器
    trainer = Trainer(model, config, train_loader)
    
    print(f"\n开始训练...")
    history = trainer.train()
    
    print("\n✅ 训练器测试通过！")
