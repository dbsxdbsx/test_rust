use nalgebra as na;
use rand::distributions::{Distribution, Uniform};

pub struct Tensor {
    data: na::DMatrix<f32>,
    shape: Vec<usize>, // 用于存储Tensor的形状, 例如[]表示标量；[1]表示含还有1个元素的行向量；[2, 3]表示2行3列
}

impl Tensor {
    pub fn data(&self) -> &na::DMatrix<f32> {
        &self.data
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn empty(rows: usize, cols: usize) -> Self {
        Tensor {
            data: na::DMatrix::zeros(rows, cols),
            shape: vec![rows, cols],
        }
    }

    pub fn scalar(value: f32) -> Self {
        Tensor {
            data: na::DMatrix::from_element(1, 1, value),
            shape: Vec::new(),
        }
    }

    pub fn eye(size: usize) -> Self {
        Tensor {
            data: na::DMatrix::identity(size, size),
            shape: vec![size, size],
        }
    }

    pub fn random(rows: usize, cols: usize, min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new(min, max);
        Tensor {
            data: na::DMatrix::from_fn(rows, cols, |_, _| uniform.sample(&mut rng)),
            shape: vec![rows, cols],
        }
    }
}
