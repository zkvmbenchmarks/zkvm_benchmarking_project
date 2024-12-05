use methods::{
    TEST_PROJECT_ELF, TEST_PROJECT_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();


    // INPUT_ASSIGNMENTS

    let env = // ENVIRONMENT_BUILDER;


    let prover = default_prover();

    let prove_info = prover
        .prove(env, TEST_PROJECT_ELF)
        .unwrap();

    let receipt = prove_info.receipt;

    let output: u32 = receipt.journal.decode().unwrap();
    println!("Output: {}", output);
    receipt
        .verify(TEST_PROJECT_ID)
        .unwrap();
}