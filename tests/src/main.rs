mod codegen;
mod env_adapters;
use codegen::CodeGenerator;

fn main() {
    // get template name from command line
    let args: Vec<String> = std::env::args().collect();
    let template_name = &args[1];
    let template_path = format!("./test_templates/{}.rs", template_name);
    let generator = CodeGenerator::new(Box::new(env_adapters::Risc0Env));
    generator.generate_code(
        &template_path,
        "../risc0_benchmarks/test_project/methods/guest/src/main.rs",
        "../risc0_benchmarks/test_project/host/src/main.rs",
    );

    let risc0_generator = CodeGenerator::new(Box::new(env_adapters::Sp1Env));
    risc0_generator.generate_code(
        &template_path,
        "../sp1_benchmarks/sp1_project/program/src/main.rs",
        "../sp1_benchmarks/sp1_project/script/src/bin/main.rs",
    );
}
