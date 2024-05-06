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
