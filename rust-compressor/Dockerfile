# Use an official Rust runtime as a base image
FROM rust:1.77-slim-bookworm AS builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Cargo manifest file
COPY Cargo.toml .

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Create a minimal runtime image
FROM debian:bookworm-slim

# Install necessary runtime dependencies (if any)
# For a simple binary, you might not need any.
# If your Rust app links to system libraries, install them here.
# Example: RUN apt-get update && apt-get install -y libsqlite3-dev

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-compressor /app/

# Make the binary executable
RUN chmod +x /app/rust-compressor

# Set the entrypoint to run the binary
ENTRYPOINT ["/app/rust-compressor"]

# You can optionally define a default command if no arguments are provided
# CMD ["--help"]