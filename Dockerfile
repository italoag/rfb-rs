# Multi-stage build pinned to Rust 1.91 for edition 2024 support
ARG RUST_VERSION=1.91.0
ARG ALPINE_VERSION=3.20
FROM rust:${RUST_VERSION}-alpine3.20 AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    postgresql-dev

WORKDIR /usr/src/rfb-rs

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "" > src/lib.rs

# Build dependencies (cached layer)
RUN cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src
COPY tests ./tests

# Build application
RUN touch src/main.rs src/lib.rs && \
    cargo build --release

# Runtime stage stays on the same Alpine track for parity with builder tooling
FROM alpine:${ALPINE_VERSION}

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    libgcc \
    postgresql-client

# Create non-root user
RUN addgroup -g 1000 rfb && \
    adduser -D -s /bin/sh -u 1000 -G rfb rfb

WORKDIR /app

# Copy binary from builder
COPY --from=builder /usr/src/rfb-rs/target/release/rfb /usr/local/bin/rfb

# Set ownership
RUN chown -R rfb:rfb /app

# Switch to non-root user
USER rfb

# Create data directories
RUN mkdir -p /app/data /app/output

# Expose API port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD rfb --version || exit 1

# Default command
CMD ["rfb", "api", "--host", "0.0.0.0", "--port", "8080"]
