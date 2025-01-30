# Use the official Rust image as a builder
FROM rust:1.84 AS builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build the dependencies (this will cache the dependencies layer)
# RUN cargo build --release

# Copy the source code
COPY src/ ./src/

# Build the final release
RUN cargo build --release

# Use a smaller base image for the final image
FROM debian:bookworm-slim
#FROM rust:1.84-slim

# Install dependencies required for the app
RUN apt-get update && apt-get install -y \
    libssl-dev procps \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder image
COPY --from=builder /app/target/release/rust-test-1 /usr/local/bin/rust-test-1

# Expose the port the app will run on
EXPOSE 8080

# Run the binary
CMD ["rust-test-1"]
