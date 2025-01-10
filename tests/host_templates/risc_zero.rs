use methods::{
    TEST_PROJECT_ELF, TEST_PROJECT_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use benchmarker;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();


    // INPUT_ASSIGNMENTS

    let env = // ENVIRONMENT_BUILDER;

    let prover = default_prover();
    
    let mut benchmarker = benchmarker::Benchmarker::new();
    benchmarker.start_benchmark();
    let prove_info = prover
        .prove(env, TEST_PROJECT_ELF)
        .unwrap();
    let benchmark_results = benchmarker.end_benchmark();
    //log proving benchmark results
    if let Some(duration) = benchmark_results {
        println!("Proving time: {:?}", duration);
    }

    let receipt = prove_info.receipt;

    let serialized_receipt = bincode::serialize(&receipt).unwrap();
    let size_in_kb = serialized_receipt.len() as f64 / 1024.0;
    println!("Proof size: {} KB", size_in_kb);

    let output: u32 = receipt.journal.decode().unwrap();
    // println!("Output: {}", output);

    let mut verifying_benchmarker = benchmarker::Benchmarker::new();
    verifying_benchmarker.start_benchmark();
    receipt
        .verify(TEST_PROJECT_ID)
        .unwrap();
    let verifying_benchmark_results = verifying_benchmarker.end_benchmark();

    //logs verification benchmark results
    if let Some(duration) = verifying_benchmark_results {
        println!("Verification time: {:?}", duration);
    } 
}