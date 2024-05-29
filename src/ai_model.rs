use std::{fmt, ops, cmp};
use crate::matrix::Matrix;

#[macro_export]
macro_rules! aimodel {
    ($t:ty, $f:expr) => {
        use std::{fs::File, io::Read};

        let file = File::open($f)?;
        let mut buffer: Vec<u8> = vec![];

        file.read_to_end(&mut buffer)?;
    };
}

pub struct AIModel<T> {
    weights: Vec<Vec<Vec<T>>>,
    biases: Vec<Vec<T>>,
    activators: Vec<fn(T) -> T>,
}

impl<T: fmt::Display
    + Copy + Default + Clone
    + ops::Add<Output = T>
    + ops::Mul<Output = T>
    + ops::Div<Output = T>
    + ops::Sub<Output = T>
    + ops::AddAssign<T>
    + cmp::PartialEq<T>> AIModel<T> {
    pub fn execute(self, inputs: Vec<T>) -> Vec<T> {
        let mut current: Matrix<T> = Matrix::from(vec![inputs]).transpose();

        for i in 0..self.weights.len() {
            current = current.multiply(
                &Matrix::from(self.weights[i].clone())
            ).add(
                &Matrix::from(vec![self.biases[i].clone()]).transpose()
            ).map(&self.activators[i]);
        }

        current.data[0].clone()
    }
}
