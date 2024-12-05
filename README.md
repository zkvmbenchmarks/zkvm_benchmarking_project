# zkVM Benchmarking Project

This project benchmarks zkVMs like RISC Zero and SP1 in isolated Docker environments.

## Requirements

Each zkVM runs in its own Docker container. Ensure `docker` is installed and configured.

## Setup

Build Docker environments for RISC Zero and SP1:

```bash
bash built_docker_environments.sh

Usage
Enter Docker Environments

To run benchmarks, first enter the corresponding Docker environment.
RISC Zero Docker Environment

docker run -it --rm -v $(pwd):/app zkvm-benchmarking-risc0 bash

SP1 Docker Environment

docker run -it --rm -v $(pwd):/app zkvm-benchmarking-sp1 bash

Run Benchmarks in Docker
Run RISC Zero Benchmarks

Once inside the RISC Zero Docker environment, run:

make risc0 TEST_NAME=<test_name>

Example:

make risc0 TEST_NAME=vecSum10

Results: results/risc0_[TEST_NAME]_benchmark_results.txt
Run SP1 Benchmarks

Once inside the SP1 Docker environment, run:

make sp1 TEST_NAME=<test_name>

Example:

make sp1 TEST_NAME=fibTest

Results: results/sp1_[TEST_NAME]_benchmark_results.txt
Run All Benchmarks

Inside each Docker environment, run:

make all

Running Sample Tests
RISC Zero

Build and run a sample test in the RISC Zero Docker environment:

bash run_risc0_sample_test_in_docker.sh

SP1

Build and run a sample test in the SP1 Docker environment:

bash run_sp1_sample_test_in_docker.sh

File Overview

    built_docker_environments.sh: Build Docker containers for RISC Zero and SP1.
    Dockerfile.risc0: Dockerfile for RISC Zero.
    Dockerfile.sp1: Dockerfile for SP1.
    Makefile: Tasks for benchmarks.
    log_cleaner.sh: Cleans and formats logs.
    results/: Benchmark results.
    risc0_benchmarks/: RISC Zero projects.
    sp1_benchmarks/: SP1 projects.



