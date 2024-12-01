# Use the official Rust image as the base (Debian-based)
FROM rust:latest


# Install system dependencies
RUN apt-get update && apt-get install -y \
#    valgrind \          
    make \              
    procps \            
    curl \              
    && apt-get clean    

# Set the working directory inside the container
WORKDIR /app


RUN mkdir -p /root/.cargo/bin && \
    chmod -R 777 /root/.cargo/bin && \
    curl -L https://risczero.com/install | bash && \
    /root/.risc0/bin/rzup install


# Copy the entire project into the container
COPY . .

# Ensure scripts like log_cleaner.sh are executable
RUN chmod +x log_cleaner.sh

# Default command (optional, can be overridden)
CMD ["bash"]

