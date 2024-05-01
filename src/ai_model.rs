pub struct AIModel {
    weights: Vec<Vec<Vec<f64>>>,
    biases: Vec<Vec<f64>>,
    activators: Vec<fn(f64) -> f64>,
}
