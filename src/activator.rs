#[derive(Clone, Copy)]
pub struct Activator {
    pub function: fn(f64) -> f64,
    pub derivative: fn(f64) -> f64,
}

pub const LINEAR: Activator = Activator {
    function: |x| x,
    derivative: |_| 1.0,
};

pub const SIGMOID: Activator = Activator {
    function: |x| 1.0 / (1.0 + (-x).exp()),
    derivative: |x| (1.0 - x) * x,
};

pub const TANH: Activator = Activator {
    function: |x| (x.exp() - (-x).exp()) / (x.exp() + (-x).exp()),
    derivative: |x| 1.0 - x * x,
};

pub const RELU: Activator = Activator {
    function: |x| if x > 0.0 { x } else { 0.0 },
    derivative: |x| if x > 0.0 { 1.0 } else { 0.0 },
};
