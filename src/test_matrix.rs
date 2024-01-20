use ndarray::{arr1, arr2, Array1, Array2};

pub fn test() {
    // let a: Array2<f64> = arr2(&[[1.0, 2.0], [3.0, 4.0]]);
    let a: Array2<f64> = arr2(&[[1.0, 2.0, 3.0], [3.0, 4.0, 5.0]]);
    let b: Array2<f64> = arr2(&[[5.0, 6.0], [5.0, 6.0], [7.0, 8.0]]);
    // 执行矩阵乘法
    let _c = a.dot(&b);
    let c = b.dot(&a);
    // 打印结果矩阵
    println!("矩阵C:\n{}", c);

    // let a: Array2<f64> = arr2(&[[1.0, 2.0, 3.0]]);
    // let b: Array2<f64> = arr2(&[[5.0, 6.0, 7.0]]);
    let a: Array1<f64> = arr1(&[1.0, 2.0, 3.0]);
    let b: Array1<f64> = arr1(&[1.0, 2.0, 3.0]);
    // let b: Array2<f64> = arr2(&[[5.0], [6.0], [7.0]]);

    // 执行矩阵乘法
    let _c = a.dot(&b);
    let c = b.dot(&a);
    // 打印结果矩阵
    println!("矩阵C:\n{}", c);
}
