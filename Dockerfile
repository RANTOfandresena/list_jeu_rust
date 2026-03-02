# ---------------- BUILD ----------------
FROM rust:1.93.1-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copier tout le workspace directement
COPY . .

# Build release
RUN cargo build --release

# ---------------- RUNTIME ----------------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/backend .

EXPOSE 8090

CMD ["./backend"]
