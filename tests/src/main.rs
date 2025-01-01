mod codegen;
mod env_adapters;
use codegen::CodeGenerator;

fn main() {
    // get template name from command line
    let args: Vec<String> = std::env::args().collect();
    let template_name = &args[1];
    let template_path = format!("./test_templates/{}", template_name);
    // check if reset flag is set
    let mut reset_flag: bool = false;
    if args.len() > 2 {
        reset_flag = &args[2] == "true";
    } 

    let generator = CodeGenerator::new(Box::new(env_adapters::Risc0Env));
    generator.generate_code(
        &template_path,
        "../risc0_benchmarks/test_project/methods/guest/src",
        "../risc0_benchmarks/test_project/host/src",
    );

    let sp1_generator = CodeGenerator::new(Box::new(env_adapters::Sp1Env));
    sp1_generator.generate_code(
        &template_path,
        "../sp1_benchmarks/sp1_project/program/src",
        "../sp1_benchmarks/sp1_project/script/src/bin",
    );

    if reset_flag {
        generator.reset();
        sp1_generator.reset();
    }
}
