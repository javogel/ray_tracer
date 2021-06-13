pub struct Matrix{
    pub data: Vec<Vec<f32>>
}



pub fn matrix(x: usize, y: usize) -> Matrix {
    Matrix {data: vec![vec![0.0; x]; y]}
}