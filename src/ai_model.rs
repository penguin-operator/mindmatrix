pub struct AIModel<T> {
    weights: Vec<Vec<Vec<T>>>,
    biases: Vec<Vec<T>>,
    activators: Vec<fn(T) -> T>,
}

macro_rules! aimodel {
    ($f:expr) => {
        
    };
}