use sp1_sdk::{include_elf, utils, ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use benchmarker;

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_elf!("fibonacci-program");

fn main() {
    // Setup logging.
    utils::setup_logger();

    // INPUT_ASSIGNMENTS

    // ENVIRONMENT_BUILDER

    // Create a `ProverClient` method.
    let client = ProverClient::new();

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    // let (_, report) = client.execute(ELF, stdin.clone()).run().unwrap();
    // println!("executed program with {} cycles", report.total_instruction_count());

    // Generate the proof for the given program and input.
    let (pk, vk) = client.setup(ELF);
    let mut benchmarker = benchmarker::Benchmarker::new();
    benchmarker.start_benchmark();
    let mut proof = client.prove(&pk, stdin).run().unwrap();
    let benchmark_results = benchmarker.end_benchmark();
    if let Some(duration) = benchmark_results {
        println!("Proving time: {:?}", duration);
    }
    //serialize the receipt to its bytes and log its size in kb
    let serialized_receipt = bincode::serialize(&proof).unwrap();
    let size_in_kb = serialized_receipt.len() as f64 / 1024.0;
    println!("Proof size: {} KB", size_in_kb);


    // Read and verify the output.
    let a = proof.public_values.read::<u32>();
    // println!("Output: {}", a);

    // Verify proof and public values
    let mut verifying_benchmarker = benchmarker::Benchmarker::new();
    verifying_benchmarker.start_benchmark();
    client.verify(&proof, &vk).expect("verification failed");
    let verifying_benchmark_results = verifying_benchmarker.end_benchmark();

    //logs verification benchmark results
    if let Some(duration) = verifying_benchmark_results {
        println!("Verification time: {:?}", duration);
    }
}