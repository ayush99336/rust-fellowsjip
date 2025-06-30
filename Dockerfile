# Build stage
FROM rust:1.70 as builder

WORKDIR /app

# Copy dependency files first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY tests ./tests

# Build the project
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/rust-http-server /app/solana-http-server

# Expose the port the server runs on
EXPOSE 3000

# Set the startup command
CMD ["./solana-http-server"]
