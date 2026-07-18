FROM rust:1.88-alpine

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/parser"]