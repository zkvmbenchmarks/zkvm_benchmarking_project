# built risc0 environment
docker build -t zkvm-benchmarking-risc0 -f Dockerfile.risc0 .

# built sp1 environment
docker build -t zkvm-benchmarking-sp1 -f Dockerfile.sp1 .
