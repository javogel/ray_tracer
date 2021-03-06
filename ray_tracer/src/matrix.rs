use crate::transforms::*;
use crate::tuple::*;
use crate::utils::*;
use std::cmp::PartialOrd;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Index, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Sub<T, Output = T>
        + Add<T, Output = T>
        + Neg<Output = T>
        + Copy
        + Clone
        + Into<f64>
        + Display
        + fmt::Debug,
{
    pub fn new(x: usize, y: usize) -> Matrix<f64> {
        Matrix {
            data: vec![vec![0.0; x]; y],
        }
    }
    pub fn from(input: Vec<Vec<T>>) -> Self {
        return Self { data: input };
    }

    pub fn from_tuple(t: Tuple) -> Matrix<f64> {
        let mut data = vec![];
        for i in t.raw() {
            data.push(vec![i])
        }
        Matrix { data }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        let Self { data } = self;
        (data.len(), data[0].len())
    }

    pub fn multiply_matrix(&self, other: Matrix<T>) -> Matrix<f64>
    where
        T: Mul<T, Output = f64> + Add<f64>,
    {
        let (ay, ax) = self.dimensions();
        let (by, bx) = other.dimensions();
        assert!(ax == by);
        let mut m = Matrix::<f64>::new(bx, ay);
        for row in 0..ay {
            for col in 0..bx {
                let mut total: f64 = 0.;
                for i in 0..by {
                    total = self[row][i] * other[i][col] + total;
                }
                m.data[row][col] = total;
            }
        }
        return m;
    }

    pub fn transpose(&self) -> Matrix<f64> {
        let (rows, cols) = self.dimensions();
        let mut m = Self::new(cols, rows);
        for row in 0..rows {
            for col in 0..cols {
                m.data[col][row] = self.data[row][col].clone().into()
            }
        }
        return m;
    }

    fn determinant_2x2(&self) -> T
    where
        T: Mul<T, Output = T>,
    {
        assert!(self.dimensions() == (2, 2));
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }

    fn determinant_ixj(&self) -> T
    where
        T: Mul<T, Output = T>,
    {
        let (_, columns) = self.dimensions();
        let mut cofactors = vec![];
        for i in 0..columns {
            cofactors.push(self[0][i] * self.cofactor(0, i))
        }
        return cofactors.into_iter().reduce(|a, b| a + b).unwrap();
    }

    pub fn determinant(&self) -> T
    where
        T: Mul<T, Output = T>,
    {
        match self.dimensions() {
            (2, 2) => self.determinant_2x2(),
            _ => self.determinant_ixj(),
        }
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Self {
        let (cols, rows) = self.dimensions();
        let mut data = vec![vec![]; rows - 1];
        for r in 0..rows {
            for c in 0..cols {
                let row_index = match r > row {
                    true => r - 1,
                    false => r,
                };

                if r != row && c != column {
                    data[row_index].push(self[r][c])
                }
            }
        }
        Matrix { data }
    }

    pub fn minor(&self, row: usize, column: usize) -> T
    where
        T: Mul<T, Output = T>,
    {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> T
    where
        T: Mul<T, Output = T>,
    {
        let minor = self.minor(row, column);
        return if is_odd(column + row) { -minor } else { minor };
    }

    pub fn inverse(&self) -> Option<Matrix<f64>>
    where
        T: Mul<T, Output = T>,
        f64: From<T>,
    {
        let determinant = f64::from(self.determinant());
        let (cols, rows) = self.dimensions();

        return match determinant == 0. {
            true => None,
            false => {
                let mut m = matrix(cols, rows);
                for row in 0..rows {
                    for col in 0..cols {
                        m.data[col][row] = f64::from(self.cofactor(row, col)) / determinant
                    }
                }
                Some(m)
            }
        };
    }
}

impl Matrix<Scalar> {
    pub fn multiply_tuple(&self, other: Tuple) -> Tuple {
        let m = Matrix::<f64>::from_tuple(other);
        let result = self.multiply_matrix(m);

        Tuple::from_matrix(result)
    }
}

impl Transformations for Matrix<f64> {
    fn rotate_x(self, r: f64) -> Matrix<f64> {
        rotation_x(r).multiply_matrix(self)
    }

    fn rotate_y(self, r: f64) -> Matrix<f64> {
        rotation_y(r).multiply_matrix(self)
    }

    fn rotate_z(self, r: f64) -> Matrix<f64> {
        rotation_z(r).multiply_matrix(self)
    }

    fn scale(self, x: f64, y: f64, z: f64) -> Matrix<f64> {
        scaling(x, y, z).multiply_matrix(self)
    }

    fn translate(self, x: f64, y: f64, z: f64) -> Matrix<f64> {
        translation(x, y, z).multiply_matrix(self)
    }

    fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix<f64> {
        shearing(xy, xz, yx, yz, zx, zy).multiply_matrix(self)
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> PartialEq for Matrix<T>
where
    T: PartialOrd<f64>
        + Sub<T, Output = T>
        + Add<T, Output = T>
        + Neg<Output = T>
        + Copy
        + Clone
        + Into<f64>
        + Display
        + fmt::Debug,
{
    fn eq(&self, other: &Matrix<T>) -> bool {
        let (x, y) = self.dimensions();
        for i in 0..x {
            for j in 0..y {
                if (self[i][j] - other[i][j]) > EPSILON {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Mul<T, Output = f64>
        + Add<f64>
        + Sub<T, Output = T>
        + Add<T, Output = T>
        + Neg<Output = T>
        + Copy
        + Clone
        + Into<f64>
        + Display
        + fmt::Debug,
{
    type Output = Matrix<f64>;

    fn mul(self, rhs: Matrix<T>) -> Matrix<f64> {
        self.multiply_matrix(rhs)
    }
}

impl Mul<Tuple> for Matrix<f64> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        self.multiply_tuple(rhs)
    }
}

impl Mul<Tuple> for &Matrix<f64> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        self.multiply_tuple(rhs)
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut m = vec![];
        for i in self.data.iter() {
            m.push(format!("{:?}", i))
        }
        write!(f, "{}", m.join("\n"))
    }
}

pub fn matrix(cols: usize, rows: usize) -> Matrix<f64> {
    Matrix::<f64>::new(cols, rows)
}

pub fn identity() -> Matrix<f64> {
    Matrix::from(vec![
        vec![1., 0., 0., 0.],
        vec![0., 1., 0., 0.],
        vec![0., 0., 1., 0.],
        vec![0., 0., 0., 1.],
    ])
}
