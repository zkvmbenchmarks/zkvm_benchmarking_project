# zkVM Benchmarking Project

This project benchmarks zkVMs like RISC Zero and SP1.

## Requirements

each zkvm in the project should be installed manually. 
TODO: can provide a script for that.

## Usage
### Run RISC Zero Benchmarks

Run benchmarks for a specific test under risc0_benchmarks:

make risc0 TEST_NAME=<test_name>

#### Example:

make risc0 TEST_NAME=fibonacci10

#### Results are saved in:

TODO: is this what we want to save?
results/risc0_test_project_results.txt

### Run SP1 Benchmarks

Run the SP1 benchmarks:

make sp1

Results are saved in:

results/sp1_results.txt

### Run All Benchmarks

Run benchmarks for both RISC Zero and SP1:

make all PROJECT=<project_name>

