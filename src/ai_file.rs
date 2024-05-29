use std::{fs::File, io::{Read, Write}, mem::size_of};
use crate::{neural_network::NeuralNetwork, ai_model::AIModel};

pub trait AIFile<T> {
    fn save(&self, path: &str);
    fn load(path: &str);
}

impl<T> AIFile<T> for NeuralNetwork<T> {
    fn save(&self, path: &str) {
        unimplemented!()
    }
    fn load(path: &str) {
        let size = size_of::<T>();
        let mut file = File::open(path).unwrap();
        let mut buffer: Vec<u8> = vec![];
        file.read(&mut buffer).unwrap();
        let layers =
            (buffer[0] as u128) << 96 |
            (buffer[1] as u128) << 80 |
            (buffer[2] as u128) << 64 |
            (buffer[3] as u128) << 48 |
            (buffer[4] as u128) << 32 |
            (buffer[5] as u128) << 16 |
            (buffer[6] as u128) << 8 |
            (buffer[7] as u128);

        file.read_to_end(&mut buffer).unwrap();
    }
}

impl<T> AIFile<T> for AIModel<T> {
    fn save(&self, path: &str) {
        unimplemented!()
    }
    fn load(path: &str) {
        unimplemented!()
    }
}
