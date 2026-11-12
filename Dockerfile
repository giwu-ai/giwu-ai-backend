# Stage 1: Build the Rust binary
FROM rust:1.75-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/giwu-ai-backend /app/giwu-ai-backend

EXPOSE 8080
CMD ["./giwu-ai-backend"]
