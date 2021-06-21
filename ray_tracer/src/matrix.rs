use crate::tuple::*;
use crate::utils::*;
use std::cmp::PartialOrd;
use std::fmt;
use std::ops::{self, Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new(x: usize, y: usize) -> Matrix<f32> {
        Matrix {
            data: vec![vec![0.0; x]; y],
        }
    }
    pub fn from(input: Vec<Vec<T>>) -> Self {
        return Self { data: input };
    }

    pub fn from_tuple(t: Tuple) -> Matrix<f32> {
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

    pub fn multiply_matrix(&self, other: Matrix<T>) -> Matrix<f32>
    where
        T: Mul<T, Output = f32> + Add<f32> + Copy,
    {
        let (ay, ax) = self.dimensions();
        let (by, bx) = other.dimensions();
        assert!(ax == by);
        let mut m = matrix(bx, ay);
        for row in 0..ay {
            for col in 0..bx {
                let mut total: f32 = 0.;
                for i in 0..by {
                    total = self[row][i] * other[i][col] + total;
                }
                m.data[row][col] = total;
            }
        }
        return m;
    }

    pub fn transpose(&self) -> Matrix<f32>
    where
        T: Into<f32> + Clone,
    {
        let (rows, cols) = self.dimensions();
        let mut m = Self::new(cols, rows);
        for row in 0..rows {
            for col in 0..cols {
                m.data[col][row] = self.data[row][col].clone().into()
            }
        }
        return m;
    }

    pub fn determinant(&self) -> T
    where
        T: Mul<T, Output = T> + Sub<T, Output = T> + Copy,
    {
        assert!(self.dimensions() == (2, 2));
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Self
    where
        T: Copy,
    {
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
        T: Mul<T, Output = T> + Sub<T, Output = T> + Copy,
    {
        self.submatrix(row, column).determinant()
    }
}
pub fn matrix(cols: usize, rows: usize) -> Matrix<f32> {
    Matrix::<f32>::new(cols, rows)
}

impl<T> ops::Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> PartialEq for Matrix<T>
where
    T: PartialOrd<f32> + Sub<T, Output = T> + Copy,
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
    T: Mul<T, Output = f32> + Add<f32> + Copy,
{
    type Output = Matrix<f32>;

    fn mul(self, rhs: Matrix<T>) -> Matrix<f32> {
        self.multiply_matrix(rhs)
    }
}

impl Mul<Tuple> for Matrix<f32> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        let m = Matrix::<f32>::from_tuple(rhs);
        let result = self.multiply_matrix(m);

        Tuple::from_matrix(result)
    }
}

impl fmt::Display for Matrix<f32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut m = vec![];
        for i in self.data.iter() {
            m.push(format!("{:?}", i))
        }
        write!(f, "{}", m.join("\n"))
    }
}
