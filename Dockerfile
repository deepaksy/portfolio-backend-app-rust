# Stage 1: Build Environment
FROM rust:alpine3.24 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src
RUN cargo clean

COPY src ./src
RUN cargo build --release

# Stage 2: Runtime Environment (Alpine-based)
FROM alpine:3.24

WORKDIR /app

# Install minimal runtime dependencies if your rust app needs them (like SSL certificates)
RUN apk add --no-cache ca-certificates libgcc

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/portfolio-backend-app .

EXPOSE 8080

CMD ["./portfolio-backend-app"]