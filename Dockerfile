FROM rust:1.71.0-slim

RUN mkdir -p /work

COPY . /work

WORKDIR /work

RUN cargo build --release

CMD ["/work/target/release/rinha-interpreter", "/var/rinha/source.rinha.json"]
