# Use Ubuntu as the base image
FROM ubuntu:latest

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*


# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory in the container
WORKDIR /app

# Copy the local Rust project files to the container
COPY . .



# Build the Rust project
RUN cargo build --release

# Expose the port your application will run on (change this if needed)
EXPOSE 30000

# Run the Rust application
CMD ["./target/release/load-balancer", ">", "out.txt"]

