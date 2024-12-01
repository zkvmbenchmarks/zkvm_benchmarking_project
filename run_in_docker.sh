#!/bin/bash
set -e # Exit if any command fails

# Build the Docker image (if not already built)
docker build -t zkvm-benchmarking .

# Run the container and execute whatever command is passed
docker run -it --rm -v $(pwd)/results:/app/results zkvm-benchmarking "$@"
