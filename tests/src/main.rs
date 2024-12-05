mod codegen;
mod env_adapters;
use codegen::CodeGenerator;

fn main() {
    let generator = CodeGenerator::new(Box::new(env_adapters::Risc0Env));
    generator.generate_code(
        "./test_templates/fibonacci.rs",
        "../risc0_benchmarks/test_project/methods/guest/src/main.rs",
        "../risc0_benchmarks/test_project/host/src/main.rs"
    );

    let risc0_generator = CodeGenerator::new(Box::new(env_adapters::Sp1Env));
    risc0_generator.generate_code("./test_templates/fibonacci.rs", "../sp1_benchmarks/sp1_project/program/src/main.rs", "../sp1_benchmarks/sp1_project/script/src/bin/main.rs"); 
}
