FROM rust:1.75 AS builder
COPY . .
RUN cargo build --release

ENV CLOUD_LOGGING=1

CMD ["./target/release/rust_test"]
