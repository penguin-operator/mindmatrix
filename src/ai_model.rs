use std::{fs::File, io::Read};

#[macro_export]
macro_rules! aimodel {
    ($f:expr) => {
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

impl<T> AIModel<T> {}
