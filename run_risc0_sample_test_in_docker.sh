#docker run --rm --privileged --ulimit nofile=65536:65536 -v "$(pwd):/app" zkvm-benchmarking-risc0 make risc0 TEST_NAME=fibonacci
docker run --rm --privileged --ulimit nofile=65536:65536 -v "$(pwd):/app" zkvm-benchmarking-risc0 make risc0 TEST_NAME=isprime
#docker run --rm --privileged --ulimit nofile=65536:65536 -v "$(pwd):/app" zkvm-benchmarking-risc0 make risc0 TEST_NAME=rsa
