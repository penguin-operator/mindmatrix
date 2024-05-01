use mindmatrix::{
    neural_network::NeuralNetwork,
    activator::{LINEAR, SIGMOID}
};

fn main() {
    let mut nn = NeuralNetwork::new(&[
        (2, SIGMOID), (3, SIGMOID), (2, SIGMOID), (2, LINEAR),
    ]);
    println!("{:?}", nn.think(vec![1.0, 0.0]));
}