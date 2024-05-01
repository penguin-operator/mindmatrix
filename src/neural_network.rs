use crate::activator::Activator;
use crate::matrix::Matrix;

pub struct NeuralNetwork {
    pub layers: Vec<usize>,
    pub activators: Vec<Activator>,
    pub weights: Vec<Matrix>,
    pub biases: Vec<Matrix>,
    pub data: Vec<Matrix>,
}
impl NeuralNetwork {
    pub fn new(setup: &[(usize, Activator)]) -> NeuralNetwork {
        let mut layers = vec![];
        let mut weights = vec![];
        let mut biases = vec![];
        let mut activators = vec![];

        for i in 0..setup.len() - 1 {
            weights.push(Matrix::random(setup[i].0, setup[i + 1].0));
            biases.push(Matrix::random(1, setup[i + 1].0));
        }

        for i in 0..setup.len() {
            layers.push(setup[i].0);
            activators.push(setup[i].1);
        }

        NeuralNetwork {
            layers,
            activators,
            weights,
            biases,
            data: vec![],
        }
    }
    pub fn think(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        self.data.clear();
        self.data.push(Matrix::from(vec![inputs]));

        for i in 0..self.layers.len() - 1 {
            self.data.push(
                self.data[i]
                .dot(&self.weights[i])
                .add(&self.biases[i])
                .map(self.activators[i].function)
            );
        }

        self.data[self.data.len() - 1].data[0].clone()
    }
    pub fn learn(&mut self, inputs: Vec<f64>, targets: Vec<f64>, rate: f64) {}
}
