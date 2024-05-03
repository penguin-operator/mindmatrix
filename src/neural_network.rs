use std::{fmt, ops, cmp};
use rand::distributions::{Distribution, Standard};

use crate::layer::Layer;
use crate::matrix::Matrix;

pub struct NeuralNetwork<T> {
    pub layers: Vec<Layer<T>>,
    pub weights: Vec<Matrix<T>>,
    pub biases: Vec<Matrix<T>>,
    pub data: Vec<Matrix<T>>,
}

impl<T: fmt::Display
    + Copy + Default + Clone
    + ops::Add<Output = T>
    + ops::Mul<Output = T>
    + ops::Div<Output = T>
    + ops::Sub<Output = T>
    + ops::AddAssign<T>
    + cmp::PartialEq<T>> NeuralNetwork<T> {
    pub fn new(setup: Vec<Layer<T>>) -> NeuralNetwork<T> where Standard: Distribution<T> {
        let mut layers = vec![];
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..setup.len() - 1 {
            weights.push(Matrix::random(setup[i + 1].size, setup[i].size));
            biases.push(Matrix::random(setup[i + 1].size, 1));
        }

        for i in 0..setup.len() {
            layers.push(setup[i]);
        }

        NeuralNetwork {
            layers,
            weights,
            biases,
            data: vec![],
        }
    }
    pub fn think(&mut self, inputs: Vec<T>) -> Vec<T> {
        self.data.clear();
        self.data.push(Matrix::from(vec![inputs]).transpose());

        for i in 0..self.layers.len() - 1 {
            self.data.push(
                self.weights[i]
                .dot(&self.data[i])
                .add(&self.biases[i])
                .map(&self.layers[i].function)
            );
        }

        self.data[self.data.len() - 1].data[0].clone()
    }
    pub fn learn(&mut self, outputs: Vec<T>, targets: Vec<T>, rate: T) {
        let parsed = Matrix::from(vec![outputs]).transpose();
        let mut errors = Matrix::from(vec![targets]).transpose().sub(&parsed);
        let mut gradients = parsed.map(&self.layers[self.layers.len() - 1].derivative);

        for i in (0..self.data.len() - 1).rev() {
            gradients = gradients.mul(&errors).map(&|x| x * rate);
            self.weights[i] = self.weights[i].add(&gradients.dot(&self.data[i].transpose()));
            self.biases[i] = self.biases[i].add(&gradients);
            errors = self.weights[i].transpose().dot(&errors);
            gradients = self.data[i].map(&self.layers[i].derivative);
        }
    }
    pub fn train(&mut self, inputs: Vec<Vec<T>>, targets: Vec<Vec<T>>, epochs: usize, rate: T) {
        for i in 1..=epochs {
            if epochs < 100 || i % (epochs / 100) == 0 {
                println!("epoch {i}/{epochs}");
            }
            for j in 0..inputs.len() {
                let outputs = self.think(inputs[j].clone());
                self.learn(outputs, targets[j].clone(), rate);
            }
        }
    }
}
