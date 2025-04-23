FROM rust:1.77-slim-bookworm AS builder

WORKDIR /usr/src/app

COPY Cargo.toml .


COPY src ./src


RUN cargo build --release


FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/rust-compressor /app/

RUN chmod +x /app/rust-compressor

ENTRYPOINT ["/app/rust-compressor"]
