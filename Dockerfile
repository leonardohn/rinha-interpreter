FROM rust:1.71.0-slim

WORKDIR /work

COPY . .

RUN cargo build --release

CMD ["/work/target/release/rinha-interpreter", "/var/rinha/source.rinha.json"]
