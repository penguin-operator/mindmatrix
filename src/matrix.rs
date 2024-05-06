use std::{fmt, ops, cmp};
use rand::{distributions::{Distribution, Standard}, thread_rng, Rng};

#[derive(Clone)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<T>>,
}

impl<T: fmt::Display
    + Copy + Default + Clone
    + ops::Add<Output = T>
    + ops::Mul<Output = T>
    + ops::Div<Output = T>
    + ops::Sub<Output = T>
    + ops::AddAssign<T>
    + cmp::PartialEq<T>> Matrix<T> {
    pub fn zeros(rows: usize, cols: usize) -> Matrix<T> {
        Matrix {
            rows: rows,
            cols: cols,
            data: vec![vec![T::default(); cols]; rows],
        }
    }
    pub fn from(data: Vec<Vec<T>>) -> Matrix<T> {
        let rows = data.len();
        let cols = data[0].len();
        let mut vec = Vec::new();
    
        for i in 0..rows {
            vec.push(vec![]);
            for j in 0..cols {
                vec[i].push(data[i][j]);
            }
        }
    
        Matrix {
            rows: rows,
            cols: cols,
            data: vec,
        }
    }
    pub fn random(rows: usize, cols: usize) -> Matrix<T> where Standard: Distribution<T> {
        let mut rand = thread_rng();
        let mut data = vec![vec![]; rows];
    
        for i in 0..rows {
            for _ in 0..cols {
                data[i].push(rand.gen::<T>());
            }
        }
    
        Matrix {
            rows: rows,
            cols: cols,
            data: data,
        }
    }

    pub fn multiply(&self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.cols, other.rows);
        let mut result = Matrix::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = T::default();
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }
    pub fn transpose(&self) -> Matrix<T> {
        let mut result = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
    pub fn map(&self, function: &dyn Fn(T) -> T) -> Matrix<T> {
        let mut result = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = function(self.data[i][j]);
            }
        }
        result
    }

    pub fn add(&self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);

        let mut result = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }
    pub fn sub(&self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);

        let mut result = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result
    }
    pub fn dot(&self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);

        let mut result = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        result
    }
    pub fn div(&self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);

        let mut result = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] / other.data[i][j];
            }
        }
        result
    }
}

impl<T: PartialEq<T>> cmp::PartialEq<Matrix<T>> for Matrix<T> {
    fn eq(&self, other: &Matrix<T>) -> bool {
        self.data == other.data
    }
    fn ne(&self, other: &Matrix<T>) -> bool {
        self.data != other.data
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.rows {
            write!(f, "{}[", if i == 0 { "[" } else { " " } )?;
            for j in 0..self.cols {
                write!(f, "{:5.2}", self.data[i][j])?;
                if j < self.cols - 1 { write!(f, ", ")?; }
            }
            writeln!(f, "]{}", if i < self.rows - 1 { "," } else { "]" } )?;
        }
        Ok(())
    }
}
