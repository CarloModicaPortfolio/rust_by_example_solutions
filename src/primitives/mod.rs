pub mod matrix_utils;
use std::fmt;

#[derive(Debug)]
pub struct Matrix(pub f32, pub f32, pub f32, pub f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{

        write!(f, "( {} {} ) \n( {} {} )",
               self.0, self.1, self.2, self.3)
    }
}

