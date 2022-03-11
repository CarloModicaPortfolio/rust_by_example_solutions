use super::Matrix;

pub fn transpose(matrix: Matrix) -> Matrix{
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}