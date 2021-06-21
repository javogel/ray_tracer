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
