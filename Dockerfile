# Stage 1: Build Environment
FROM rust:alpine3.24 AS builder

WORKDIR /app

# Trick to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Build actual source code
COPY src ./src
RUN cargo build --release

# Stage 2: Runtime Environment (Alpine-based)
FROM alpine:3.24

WORKDIR /app

# 1. Install runtime dependencies, including jq and util-linux (provides runuser)
RUN apk add --no-cache ca-certificates libgcc jq util-linux

# 2. Create the system appuser and appgroup that entrypoint.sh switches to
RUN addgroup -S appuser && adduser -S appuser -G appuser

# 3. Copy the compiled Rust binary from the builder stage
COPY --from=builder /app/target/release/portfolio-backend-app .

# 4. Copy and prepare the entrypoint script
COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

EXPOSE 8080

# 5. Route execution through entrypoint.sh while keeping the binary as the command
ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["./portfolio-backend-app"]
