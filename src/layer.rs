#[derive(Clone, Copy)]
pub struct Layer<T> {
    pub function: fn(T) -> T,
    pub derivative: fn(T) -> T,
    pub size: usize
}
