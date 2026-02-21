// src/utils/math.rs - 数学工具函数

/// Sigmoid 函数
pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

/// ReLU 函数
pub fn relu(x: f32) -> f32 {
    x.max(0.0)
}

/// GELU 激活函数（Transformer 常用）
pub fn gelu(x: f32) -> f32 {
    let c = 0.044715;
    x * (1.0 + (c * x * x).tanh()) * 0.5
}

/// Softmax 函数
pub fn softmax(arr: &[f32]) -> Vec<f32> {
    let max_val = arr.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = arr.iter().map(|&x| (x - max_val).exp()).collect();
    let sum: f32 = exps.iter().sum();
    exps.iter().map(|&x| x / sum).collect()
}

/// 计算交叉熵损失
pub fn cross_entropy_loss(predictions: &[f32], target: usize) -> f32 {
    let eps = 1e-10;
    let pred = predictions[target].max(eps).min(1.0 - eps);
    -pred.ln()
}

/// 矩阵乘法（简化版）
pub fn matmul(a: &[f32], b: &[f32], a_rows: usize, a_cols: usize, b_cols: usize) -> Vec<f32> {
    let mut result = vec![0.0; a_rows * b_cols];

    for i in 0..a_rows {
        for j in 0..b_cols {
            for k in 0..a_cols {
                result[i * b_cols + j] += a[i * a_cols + k] * b[k * b_cols + j];
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid() {
        assert!((sigmoid(0.0) - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_relu() {
        assert_eq!(relu(-5.0), 0.0);
        assert_eq!(relu(5.0), 5.0);
    }

    #[test]
    fn test_softmax() {
        let arr = vec![1.0, 2.0, 3.0];
        let result = softmax(&arr);
        let sum: f32 = result.iter().sum();
        assert!((sum - 1.0).abs() < 0.01);
    }
}
