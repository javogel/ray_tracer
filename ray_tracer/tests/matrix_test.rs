use ray_tracer::matrix::*;
use ray_tracer::tuple::*;

#[test]
fn test_matrix() {
    let width = 2;
    let height = 24;
    let Matrix { data } = matrix(width, height);

    assert_eq!(data.len(), height);
    assert_eq!(data[0].len(), width);
}

#[test]
fn test_matrix_from_4x4() {
    let m = Matrix::from(vec![
        vec![1., 2., 3., 4.],
        vec![5.5, 6.5, 7.5, 8.5],
        vec![9., 10., 11., 12.],
        vec![13.5, 14.5, 15.5, 16.5],
    ]);

    assert_eq!(m[0][0], 1.);
    assert_eq!(m[0][3], 4.);
    assert_eq!(m[1][0], 5.5);
    assert_eq!(m[1][2], 7.5);
    assert_eq!(m[2][2], 11.);
    assert_eq!(m[3][0], 13.5);
    assert_eq!(m[3][2], 15.5);
}

#[test]
fn test_matrix_from_2x2() {
    let m = Matrix::from(vec![vec![-3., 5.], vec![1., -2.]]);

    assert_eq!(m[0][0], -3.);
    assert_eq!(m[0][1], 5.);
    assert_eq!(m[1][0], 1.);
    assert_eq!(m[1][1], -2.);
}
#[test]
fn test_matrix_from_3x3() {
    let m = Matrix::from(vec![
        vec![-3., 5., 0.],
        vec![1., -2., -7.],
        vec![0., 1., 1.],
    ]);

    assert_eq!(m[0][0], -3.);
    assert_eq!(m[1][1], -2.);
    assert_eq!(m[2][2], 1.);
}

#[test]
fn test_matrix_equality() {
    let a = Matrix::from(vec![
        vec![1., 2., 3., 4.],
        vec![5., 6., 7., 8.],
        vec![9., 8., 7., 6.],
        vec![5., 4., 3., 2.],
    ]);

    let b = Matrix::from(vec![
        vec![1., 2., 3., 4.],
        vec![5., 6., 7., 8.],
        vec![9., 8., 7., 6.],
        vec![5., 4., 3., 2.],
    ]);

    let c = Matrix::from(vec![
        vec![2., 3., 4., 5.],
        vec![6., 7., 8., 9.],
        vec![8., 7., 6., 5.],
        vec![4., 3., 2., 1.],
    ]);

    assert!(a == b);
    assert!(a != c);
}

#[test]
fn test_matrix_multiplication() {
    let a = Matrix::from(vec![
        vec![1., 2., 3., 4.],
        vec![5., 6., 7., 8.],
        vec![9., 8., 7., 6.],
        vec![5., 4., 3., 2.],
    ]);

    let b = Matrix::from(vec![
        vec![-2., 1., 2., 3.],
        vec![3., 2., 1., -1.],
        vec![4., 3., 6., 5.],
        vec![1., 2., 7., 8.],
    ]);

    let c = Matrix::from(vec![
        vec![20., 22., 50., 48.],
        vec![44., 54., 114., 108.],
        vec![40., 58., 110., 102.],
        vec![16., 26., 46., 42.],
    ]);

    assert_eq!(a * b, c);
}

#[test]
fn test_matrix_x_tuple_multiplication() {
    let a = Matrix::from(vec![
        vec![1., 2., 3., 4.],
        vec![2., 4., 4., 2.],
        vec![8., 6., 4., 1.],
        vec![0., 0., 0., 1.],
    ]);

    let p = point(1., 2., 3.);

    assert!(a * p == point(18., 24., 33.));
}

#[test]
fn test_identity_matrix_multiplication() {
    let a = Matrix::from(vec![
        vec![0., 1., 2., 4.],
        vec![1., 2., 4., 8.],
        vec![2., 4., 8., 16.],
        vec![4., 8., 16., 32.],
    ]);

    let identity = Matrix::from(vec![
        vec![1., 0., 0., 0.],
        vec![0., 1., 0., 0.],
        vec![0., 0., 1., 0.],
        vec![0., 0., 0., 1.],
    ]);

    let p = point(1., 2., 3.);

    assert!(a.clone() * identity.clone() == a);
    assert!(identity * p == p);
}

#[test]
fn test_matrix_transpose() {
    let a = Matrix::from(vec![
        vec![0., 9., 3., 0.],
        vec![9., 8., 0., 8.],
        vec![1., 8., 5., 3.],
        vec![0., 0., 5., 8.],
    ]);

    let a_transposed = Matrix::from(vec![
        vec![0., 9., 1., 0.],
        vec![9., 8., 8., 0.],
        vec![3., 0., 5., 5.],
        vec![0., 8., 3., 8.],
    ]);

    assert!(a.transpose() == a_transposed);
}

#[test]
fn test_identity_matrix_transpose() {
    let identity = Matrix::from(vec![
        vec![1., 0., 0., 0.],
        vec![0., 1., 0., 0.],
        vec![0., 0., 1., 0.],
        vec![0., 0., 0., 1.],
    ]);

    assert!(identity.transpose() == identity);
}

#[test]
fn test_determinant() {
    let m = Matrix::from(vec![vec![1., 5.], vec![-3., 2.]]);

    assert_eq!(m.determinant(), 17.);
}

#[test]
fn test_submatrix() {
    let a = Matrix::from(vec![vec![1., 5., 0.], vec![-3., 2., 7.], vec![0., 6., 3.]]);
    let submatrix_a = Matrix::from(vec![vec![-3., 2.], vec![0., 6.]]);

    assert_eq!(a.submatrix(0, 2), submatrix_a);

    let b = Matrix::from(vec![
        vec![-6., 1., 1., 6.],
        vec![-8., 5., 8., 6.],
        vec![-1., 0., 8., 2.],
        vec![-7., 1., -1., -1.],
    ]);
    let submatrix_b = Matrix::from(vec![
        vec![-6., 1., 6.],
        vec![-8., 8., 6.],
        vec![-7., -1., -1.],
    ]);

    assert_eq!(b.submatrix(2, 1), submatrix_b);
}

#[test]
fn test_minors() {
    let a = Matrix::from(vec![
        vec![3., 5., 0.],
        vec![2., -1., -7.],
        vec![6., -1., 5.],
    ]);

    assert_eq!(a.minor(1, 0), 25.);
}

#[test]
fn test_cofactors() {
    let a = Matrix::from(vec![
        vec![3., 5., 0.],
        vec![2., -1., -7.],
        vec![6., -1., 5.],
    ]);

    assert_eq!(a.minor(0, 0), -12.);
    assert_eq!(a.cofactor(0, 0), -12.);
    assert_eq!(a.minor(1, 0), 25.);
    assert_eq!(a.cofactor(1, 0), -25.);
}

#[test]
fn test_determinant_of_3x3() {
    let a = Matrix::from(vec![vec![1., 2., 6.], vec![-5., 8., -4.], vec![2., 6., 4.]]);

    assert_eq!(a.cofactor(0, 0), 56.);
    assert_eq!(a.cofactor(0, 1), 12.);
    assert_eq!(a.cofactor(0, 2), -46.);
    assert_eq!(a.determinant(), -196.);
}

#[test]
fn test_determinant_of_4x4() {
    let a = Matrix::from(vec![
        vec![-2., -8., 3., 5.],
        vec![-3., 1., 7., 3.],
        vec![1., 2., -9., 6.],
        vec![-6., 7., 7., -9.],
    ]);

    assert_eq!(a.cofactor(0, 0), 690.);
    assert_eq!(a.cofactor(0, 1), 447.);
    assert_eq!(a.cofactor(0, 2), 210.);
    assert_eq!(a.cofactor(0, 3), 51.);
    assert_eq!(a.determinant(), -4071.);
}

#[test]
fn test_is_invertible() {
    let a = Matrix::from(vec![
        vec![6., 4., 4., 4.],
        vec![5., 5., 7., 6.],
        vec![4., -9., 3., -7.],
        vec![9., 1., 7., -6.],
    ]);

    assert_eq!(a.determinant(), -2120.);
    assert_eq!(a.inverse().is_some(), true);

    let b = Matrix::from(vec![
        vec![-4., 2., -2., -3.],
        vec![9., 6., 2., 6.],
        vec![0., -5., 1., -5.],
        vec![0., 0., 0., -0.],
    ]);

    assert_eq!(b.determinant(), 0.);
    assert_eq!(b.inverse().is_some(), false);
}

#[test]
fn test_inversion() {
    let a = Matrix::from(vec![
        vec![-5., 2., 6., -8.],
        vec![1., -5., 1., 8.],
        vec![7., 7., -6., -7.],
        vec![1., -3., 7., 4.],
    ]);

    let inverse = Matrix::from(vec![
        vec![0.21805, 0.45113, 0.24060, -0.04511],
        vec![-0.80827, -1.45677, -0.44361, 0.52068],
        vec![-0.07895, -0.22368, -0.05263, 0.19737],
        vec![-0.52256, -0.81391, -0.30075, 0.30639],
    ]);

    assert_eq!(a.cofactor(1, 0), 240.);
    assert_eq!(a.determinant(), 532.);
    assert_eq!(a.inverse().unwrap(), inverse);

    let b = Matrix::from(vec![
        vec![8., -5., 9., 2.],
        vec![7., 5., 6., 1.],
        vec![-6., 0., 9., 6.],
        vec![-3., 0., -9., -4.],
    ]);

    let inverse_b = Matrix::from(vec![
        vec![-0.15385, -0.15385, -0.28205, -0.53846],
        vec![-0.07692, 0.12308, 0.02564, 0.03077],
        vec![0.35897, 0.35897, 0.43590, 0.92308],
        vec![-0.69231, -0.69231, -0.76923, -1.92308],
    ]);

    assert_eq!(b.inverse().unwrap(), inverse_b);

    let c = Matrix::from(vec![
        vec![9., 3., 0., 9.],
        vec![-5., -2., -6., -3.],
        vec![-4., 9., 6., 4.],
        vec![-7., 6., 6., 2.],
    ]);

    let inverse_c = Matrix::from(vec![
        vec![-0.04074, -0.07778, 0.14444, -0.22222],
        vec![-0.07778, 0.03333, 0.36667, -0.33333],
        vec![-0.02901, -0.14630, -0.10926, 0.12963],
        vec![0.17778, 0.06667, -0.26667, 0.33333],
    ]);

    assert_eq!(c.inverse().unwrap(), inverse_c);
}

#[test]
fn test_multiply_product_by_inverse() {
    let a = Matrix::from(vec![
        vec![3., -9., 7., 3.],
        vec![3., -8., 2., -9.],
        vec![-4., 4., 4., 1.],
        vec![-6., 5., -1., 1.],
    ]);

    let b = Matrix::from(vec![
        vec![8., 2., 2., 2.],
        vec![3., -1., 7., 9.],
        vec![7., 0., 5., 4.],
        vec![6., -2., 0., 5.],
    ]);

    let c = a.clone() * b.clone();

    assert_eq!(c * b.inverse().unwrap(), a);
}
