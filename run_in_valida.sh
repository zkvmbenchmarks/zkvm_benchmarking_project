# Download the container
docker pull ghcr.io/lita-xyz/llvm-valida-releases/valida-build-container:v0.7.0-alpha-amd64

# cd your-valida-project

# Enter the container:
docker run --platform linux/amd64 -it --rm -v $(realpath .):/src ghcr.io/lita-xyz/llvm-valida-releases/valida-build-container:v0.7.0-alpha-amd64

# You are now in a shell with the valida rust toolchain installed!
