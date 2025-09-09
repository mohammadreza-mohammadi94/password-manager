# Build stage
FROM rust:1.70-slim as builder

WORKDIR /usr/src/password-manager
COPY . .

RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/password-manager/target/release/password-manager .

# Create volume for persistent storage
VOLUME /root/.password-manager

CMD ["password-manager"]
