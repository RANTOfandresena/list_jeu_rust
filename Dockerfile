# Stage 1: Build
FROM rust:latest as builder

WORKDIR /app

# Copy the entire project
COPY . .

# Build the project
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install required dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/backend /app/backend

# Expose the port
EXPOSE 8090

# Run the application
CMD ["./backend"]
