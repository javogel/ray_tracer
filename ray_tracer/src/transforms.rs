use crate::matrix::Matrix;

pub trait Transformations {
    fn rotate_x(self, r: f64) -> Self;
    fn rotate_y(self, r: f64) -> Self;
    fn rotate_z(self, r: f64) -> Self;
    fn scale(self, x: f64, y: f64, z: f64) -> Self;
    fn translate(self, x: f64, y: f64, z: f64) -> Self;
    fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self;
}

pub fn translation(x: f64, y: f64, z: f64) -> Matrix<f64> {
    Matrix::from(vec![
        vec![1., 0., 0., x],
        vec![0., 1., 0., y],
        vec![0., 0., 1., z],
        vec![0., 0., 0., 1.],
    ])
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix<f64> {
    Matrix::from(vec![
        vec![x, 0., 0., 0.],
        vec![0., y, 0., 0.],
        vec![0., 0., z, 0.],
        vec![0., 0., 0., 1.],
    ])
}

pub fn rotation_x(r: f64) -> Matrix<f64> {
    Matrix::from(vec![
        vec![1., 0., 0., 0.],
        vec![0., r.cos(), -r.sin(), 0.],
        vec![0., r.sin(), r.cos(), 0.],
        vec![0., 0., 0., 1.],
    ])
}

pub fn rotation_y(r: f64) -> Matrix<f64> {
    Matrix::from(vec![
        vec![r.cos(), 0., r.sin(), 0.],
        vec![0., 1., 0., 0.],
        vec![-r.sin(), 0., r.cos(), 0.],
        vec![0., 0., 0., 1.],
    ])
}

pub fn rotation_z(r: f64) -> Matrix<f64> {
    Matrix::from(vec![
        vec![r.cos(), -r.sin(), 0., 0.],
        vec![r.sin(), r.cos(), 0., 0.],
        vec![0., 0., 1., 0.],
        vec![0., 0., 0., 1.],
    ])
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix<f64> {
    Matrix::from(vec![
        vec![1., xy, xz, 0.],
        vec![yx, 1., yz, 0.],
        vec![zx, zy, 1., 0.],
        vec![0., 0., 0., 1.],
    ])
}
