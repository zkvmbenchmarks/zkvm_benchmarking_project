mod codegen;
mod env_adapters;
use codegen::CodeGenerator;

fn main() {
    let generator = CodeGenerator::new(Box::new(env_adapters::Risc0Env));
    generator.generate_code(
        "./test_templates/fibonacci.rs",
        "../risc0_benchmarks/test_project/methods/guest/src/main.rs",
    );

    let risc0_generator = CodeGenerator::new(Box::new(env_adapters::Sp1Env));
    risc0_generator.generate_code("./test_templates/fibonacci.rs", "output_risc0.rs"); // TODO
}
