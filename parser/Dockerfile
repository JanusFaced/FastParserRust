FROM rust:1.88-alpine AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
COPY src/ ./src/
RUN touch src/main.rs && cargo build --release

FROM alpine:3.18
WORKDIR /app
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/parser /app/parser
RUN chmod +x /app/parser
EXPOSE 8080
CMD ["/app/parser"]
