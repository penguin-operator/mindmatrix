use std::cmp;
use rand::{thread_rng, Rng};

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows: rows,
            cols: cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }
    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
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
    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rand = thread_rng();
        let mut data = vec![vec![]; rows];
    
        for i in 0..rows {
            for _ in 0..cols {
                data[i].push(rand.gen::<f64>() * 2.0 - 1.0);
            }
        }
    
        Matrix {
            rows: rows,
            cols: cols,
            data: data,
        }
    }

    pub fn dot(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);
        let mut result = Matrix::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
    pub fn map(&self, function: fn(f64) -> f64) -> Matrix {
        let mut result = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = function(self.data[i][j]);
            }
        }
        result
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
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
    pub fn sub(&self, other: &Matrix) -> Matrix {
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
    pub fn mul(&self, other: &Matrix) -> Matrix {
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
    pub fn div(&self, other: &Matrix) -> Matrix {
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

impl cmp::PartialEq<Matrix> for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        self.data == other.data
    }
    fn ne(&self, other: &Matrix) -> bool {
        self.data != other.data
    }
}