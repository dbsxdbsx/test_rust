mod tensor;
pub use tensor::Tensor;

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra as na;

    #[test]
    fn test_empty() {
        // 测试创建一个2x3的空Tensor，数据全部为 0.0
        let tensor = Tensor::empty(2, 3);
        assert_eq!(
            tensor.data(),
            &na::DMatrix::from_row_slice(2, 3, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0])
        );

        // 测试创建一个1x3的空Tensor，数据全部为 0.0
        let tensor = Tensor::empty(1, 3);
        assert_eq!(
            tensor.data(),
            &na::DMatrix::from_row_slice(1, 3, &[0.0, 0.0, 0.0])
        );

        // 测试创建一个2x1的空Tensor，数据全部为 0.0
        let tensor = Tensor::empty(2, 1);
        assert_eq!(
            tensor.data(),
            &na::DMatrix::from_row_slice(2, 1, &[0.0, 0.0])
        );
    }

    #[test]
    fn test_random() {
        let tensor = Tensor::random(2, 3, 0.0, 1.0);
        assert_eq!(tensor.shape(), &[2, 3]);
        assert!(tensor.data().iter().all(|&x| x >= 0.0 && x <= 1.0));
    }

    #[test]
    fn test_eye() {
        let tensor = Tensor::eye(1);
        assert_eq!(tensor.data(), &na::DMatrix::from_row_slice(1, 1, &[1.0]));

        let tensor = Tensor::eye(3);
        assert_eq!(
            tensor.data(),
            &na::DMatrix::from_row_slice(3, 3, &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
        );
    }
}
