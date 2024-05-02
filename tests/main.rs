use rand::distributions::Distribution;
use mindmatrix::{
    neural_network::NeuralNetwork,
    layer::Layer
};

fn sigmoid(size: usize) -> Layer<f64> {
    Layer {
        function: |x| 1.0 / (1.0 + (-x).exp()),
        derivative: |x| x * (1.0 - x),
        size
    }
}

fn linear(size: usize) -> Layer<f64> {
    Layer {
        function: |x| x,
        derivative: |x| 1.0,
        size
    }
}

fn main() {
    let mut xor_nn: NeuralNetwork<f64> = NeuralNetwork::new(vec![
        sigmoid(2), sigmoid(3), linear(1),
    ]);

    let x = vec![
        vec![0.0, 0.0],
        vec![1.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 1.0],
    ];
    let y = vec![
        vec![0.0],
        vec![1.0],
        vec![1.0],
        vec![0.0],
    ];

    xor_nn.train(x, y, 10000, 0.4);

    println!("{:?}", xor_nn.think(vec![0.0, 0.0]));
    println!("{:?}", xor_nn.think(vec![1.0, 0.0]));
    println!("{:?}", xor_nn.think(vec![0.0, 1.0]));
    println!("{:?}", xor_nn.think(vec![1.0, 1.0]));
}
