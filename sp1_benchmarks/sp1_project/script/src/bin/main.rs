use sp1_sdk :: { include_elf , utils , ProverClient , SP1ProofWithPublicValues , SP1Stdin } ;
use benchmarker ;
# [doc = " The ELF we want to execute inside the zkVM."] const ELF : & [u8] = include_elf ! ("fibonacci-program") ;
fn main () { utils :: setup_logger () ; let input1 = input () ; let input2 = other_input (7) ; let mut stdin = SP1Stdin :: new () ; stdin . write (& input1) ; stdin . write (& input2) ; let client = ProverClient :: new () ; let (pk , vk) = client . setup (ELF) ; let mut benchmarker = benchmarker :: Benchmarker :: new () ; benchmarker . start_benchmark () ; let mut proof = client . prove (& pk , stdin) . run () . unwrap () ; let benchmark_results = benchmarker . end_benchmark () ; if let Some ((duration , peak_memory)) = benchmark_results { println ! ("Proving time: {:?}" , duration) ; println ! ("Peak memory consumption during proving: {} KB" , peak_memory) ; } let serialized_receipt = bincode :: serialize (& proof) . unwrap () ; let size_in_kb = serialized_receipt . len () as f64 / 1024.0 ; println ! ("Proof size: {} KB" , size_in_kb) ; let a = proof . public_values . read :: < u32 > () ; let mut verifying_benchmarker = benchmarker :: Benchmarker :: new () ; verifying_benchmarker . start_benchmark () ; client . verify (& proof , & vk) . expect ("verification failed") ; let verifying_benchmark_results = verifying_benchmarker . end_benchmark () ; if let Some ((duration , peak_memory)) = verifying_benchmark_results { println ! ("Verification time: {:?}" , duration) ; println ! ("Peak memory consumption during verification: {} KB" , peak_memory) ; } }
fn input () -> u32 { 10 }
fn other_input (n : usize) -> f64 { (2 * n) as f64 }
fn other_host_function () -> f64 { 1.0 }
