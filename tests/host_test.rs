use methods::{TEST_PROJECT_ELF, TEST_PROJECT_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
    let input1 = input();
    let input2 = other_input(7);
    let env = ExecutorEnv::builder()
        .write(&input1)
        .write(&input2)
        .build()
        .unwrap();
    let prover = default_prover();
    let prove_info = prover.prove(env, TEST_PROJECT_ELF).unwrap();
    let receipt = prove_info.receipt;
    let output = receipt.journal.decode().unwrap();
    receipt.verify(TEST_PROJECT_ID).unwrap();
}
fn input() -> u32 {
    10
}
fn other_input() -> f64 {
    3.14
}
fn other_host_function() -> f64 {
    1.0
}
