use nalgebra::Matrix2;
use crate::env_adapters::NotImplementedEnv as env;

fn main() {
    let iterations: u32 = env::read(); // #input()
    let other_input: f64 = env::read(); // #other_input(7)
    let answer = fibonacci(iterations);
    env::commit(&answer);
}

fn fibonacci(n: u32) -> u64 {
    Matrix2::new(1, 1, 1, 0).pow(n - 1)[(0, 0)]
}

#[host]
fn input() -> u32 {
    10
}

#[host]
fn other_input() -> f64 {
    3.14
}

#[host]
fn other_host_function() -> f64 {
    1.0
}