//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use std::env;
mod benchmarker;
use bincode;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    let input = env::var("TEST_NAME").expect("TEST_NAME environment variable not set");
    println!("TEST_NAME: {}", input);

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&input);

    if args.execute {
        /* 
        // Execute the program
        let (output, report) = client.execute(FIBONACCI_ELF, stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let decoded = PublicValuesStruct::abi_decode(output.as_slice(), true).unwrap();
        let PublicValuesStruct { n, a, b } = decoded;
        println!("n: {}", n);
        println!("a: {}", a);
        println!("b: {}", b);

        let (expected_a, expected_b) = fibonacci_lib::fibonacci(n);
        assert_eq!(a, expected_a);
        assert_eq!(b, expected_b);
        println!("Values are correct!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
        */
    } else {
        // Setup the program for proving.
        //aga kod çalışıyo ama burda fibonacci setup felan yapiyo
        //burayı nasıl halletcez başka hangi setuplar var bilmiyorum bi bakmak lazım
        let (pk, vk) = client.setup(FIBONACCI_ELF);

        //starts time and peak memory benchmark measurements, proves test project, then ends benchmarks measurements
        let mut benchmarker = benchmarker::Benchmarker::new();
        benchmarker.start_benchmark();
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        let benchmark_results = benchmarker.end_benchmark();

        if let Some((duration, peak_memory)) = benchmark_results {
            println!("Proving time: {:?}", duration);
            println!("Peak memory consumption during proving: {} KB", peak_memory);
        }

        //serialize the receipt to its bytes and log its size in kb
        let serialized_receipt = bincode::serialize(&proof).unwrap();
        let size_in_kb = serialized_receipt.len() as f64 / 1024.0;
        println!("Proof size: {} KB", size_in_kb);

        //starts time and peak memory benchmarks measurements, verifies the receipt, then ends benchmarks measurements
        let mut verifying_benchmarker = benchmarker::Benchmarker::new();
        verifying_benchmarker.start_benchmark();
        client.verify(&proof, &vk).expect("failed to verify proof");
        let verifying_benchmark_results = verifying_benchmarker.end_benchmark();

        //logs verification benchmark results
        if let Some((duration, peak_memory)) = verifying_benchmark_results {
            println!("Verification time: {:?}", duration);
            println!("Peak memory consumption during verification: {} KB", peak_memory);
        }
    }
}
