//! 工具函数模块

use ndarray::Array2;
use rand::Rng;

/// 生成随机 tensor
pub fn random_tensor(shape: &[usize], low: f32, high: f32) -> Array2<f32> {
    let mut rng = rand::thread_rng();
    let size = shape.iter().product::<usize>();

    let data: Vec<f32> = (0..size).map(|_| rng.gen_range(low..high)).collect();

    Array2::from_shape_vec((shape[0], shape[1]), data).expect("Invalid shape")
}

/// 生成零 tensor
pub fn zeros(shape: (usize, usize)) -> Array2<f32> {
    Array2::zeros(shape)
}

/// 生成单位 tensor
pub fn ones(shape: (usize, usize)) -> Array2<f32> {
    Array2::ones(shape)
}

/// Softmax 函数
pub fn softmax(x: &Array2<f32>, axis: usize) -> Array2<f32> {
    let mut output = x.clone();

    if axis == 1 {
        for i in 0..x.shape()[0] {
            let row = x.row(i);
            let max_val = row.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

            let mut exp_sum = 0.0f32;
            for j in 0..x.shape()[1] {
                let exp_val = (x[[i, j]] - max_val).exp();
                output[[i, j]] = exp_val;
                exp_sum += exp_val;
            }

            for j in 0..x.shape()[1] {
                output[[i, j]] /= exp_sum;
            }
        }
    }

    output
}

/// GELU 激活函数
pub fn gelu(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| {
        let x3 = v * v * v;
        0.5 * v * (1.0 + (0.797885 * x3 / (x3.abs() + 0.044715).sqrt()).tanh())
    })
}

/// GELU 近似（更快速）
pub fn gelu_fast(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| {
        let _sqrt_2pi = (std::f32::consts::PI * 2.0).sqrt();
        0.5 * v * (1.0 + (v * 0.044715).tanh())
    })
}

/// SiLU / Swish 激活函数
pub fn silu(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| v / (1.0 + (-v).exp()))
}

/// ReLU 激活函数
pub fn relu(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| v.max(0.0))
}

/// Tanh 激活函数
pub fn tanh(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| v.tanh())
}

/// 矩阵乘法
pub fn matmul(a: &Array2<f32>, b: &Array2<f32>) -> Array2<f32> {
    a.dot(b)
}

/// 张量重塑
pub fn reshape(x: &Array2<f32>, rows: usize, cols: usize) -> Array2<f32> {
    x.clone().into_shape((rows, cols)).expect("Invalid shape")
}

/// 打印张量信息
pub fn print_tensor_info(name: &str, x: &Array2<f32>) {
    let shape = x.shape();
    let mean = x.mean().unwrap_or(0.0);
    let std = x.std(0.0);
    let min = x.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = x.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    println!(
        "{}: shape={:?}, mean={:.4}, std={:.4}, min={:.4}, max={:.4}",
        name, shape, mean, std, min, max
    );
}

/// 计算模型大小（MB）
pub fn model_size_mb(params: usize) -> f64 {
    // 假设 FP16
    params as f64 * 2.0 / (1024.0 * 1024.0)
}

/// 计算模型大小（GB）
pub fn model_size_gb(params: usize) -> f64 {
    model_size_mb(params) / 1024.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_softmax() {
        let x = Array2::from_shape_vec((2, 3), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let y = softmax(&x, 1);

        // 验证每行和为 1
        for i in 0..2 {
            let row_sum: f32 = y.row(i).iter().sum();
            assert!((row_sum - 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_gelu() {
        let x = Array2::from_shape_vec((1, 3), vec![-1.0, 0.0, 1.0]).unwrap();
        let y = gelu(&x);

        // GELU(0) = 0
        assert!((y[[0, 1]] - 0.0).abs() < 0.01);
    }
}
