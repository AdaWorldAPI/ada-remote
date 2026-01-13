# Build stage
FROM rust:1.75-slim as builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
COPY relay-server/ ./relay-server/

# Build the relay server in release mode
RUN cargo build --release -p ada-remote-relay-server

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /build/target/release/relay-server /app/relay-server

# Create a non-root user
RUN useradd -m -u 1000 ada-remote && \
    chown -R ada-remote:ada-remote /app

USER ada-remote

# Expose the default port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD [ "sh", "-c", "timeout 2 bash -c '</dev/tcp/localhost/8080' || exit 1" ]

# Run the relay server
ENTRYPOINT ["/app/relay-server"]
CMD ["--bind", "0.0.0.0:8080"]
