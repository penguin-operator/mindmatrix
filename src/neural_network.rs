use crate::activation::{Activation, LINEAR, SIGMOID, RELU, TANH};
use crate::matrix::Matrix;

pub struct NeuralNetwork {
    pub layers: Vec<usize>,
    pub weights: Vec<Matrix>,
    pub biases: Vec<Matrix>,
    pub data: Vec<Matrix>,
    pub activators: Vec<Activation>,
}
impl NeuralNetwork {
    pub fn new(input: usize, hidden: &[(Activation, usize)], output: usize) -> NeuralNetwork {
        let mut layers = vec![];
        let mut weights = vec![];
        let mut biases = vec![];
        let mut activators = vec![];

        weights.push(Matrix::random(input, hidden[0].1));
        for i in 0..hidden.len() - 1 {
            weights.push(Matrix::random(hidden[i].1, hidden[i + 1].1));
            biases.push(Matrix::zeros(1, hidden[i + 1].1));
        }
        weights.push(Matrix::random(hidden[hidden.len() - 1].1, output));
        biases.push(Matrix::zeros(1, output));

        layers.push(input);
        for i in 0..hidden.len() {
            activators.push(hidden[i].0);
            layers.push(hidden[i].1);
        }
        layers.push(output);

        NeuralNetwork {
            layers,
            weights,
            biases,
            data: vec![],
            activators,
        }
    }
}