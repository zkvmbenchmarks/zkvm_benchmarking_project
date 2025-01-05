#docker run --rm --privileged --ulimit nofile=65536:65536 -v "$(pwd):/app" zkvm-benchmarking-sp1 make sp1 TEST_NAME=fibonacci
docker run --rm --privileged --ulimit nofile=65536:65536 -v "$(pwd):/app" zkvm-benchmarking-sp1 make sp1 TEST_NAME=isprime
#docker run --rm --privileged --ulimit nofile=65536:65536 -v "$(pwd):/app" zkvm-benchmarking-sp1 make sp1 TEST_NAME=rsa
