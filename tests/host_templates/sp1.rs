use sp1_sdk::{include_elf, utils, ProverClient, SP1ProofWithPublicValues, SP1Stdin};

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
    let (_, report) = client.execute(ELF, stdin.clone()).run().unwrap();
    println!("executed program with {} cycles", report.total_instruction_count());

    // Generate the proof for the given program and input.
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, stdin).run().unwrap();

    println!("generated proof");

    // Read and verify the output.
    let a = proof.public_values.read::<u32>();

    println!("a: {}", a);

    // Verify proof and public values
    client.verify(&proof, &vk).expect("verification failed");

    println!("successfully generated and verified proof for the program!")
}