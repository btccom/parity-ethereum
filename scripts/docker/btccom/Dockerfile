FROM rust:1.37-buster AS builder

# show backtraces
ENV RUST_BACKTRACE 1

RUN apt-get update && apt-get install -y cmake libudev-dev yasm

WORKDIR /parity
COPY . /parity
RUN cargo build --release --verbose --features final
RUN strip target/release/parity


FROM debian:buster

# show backtraces
ENV RUST_BACKTRACE 1

RUN apt-get update && apt-get install -y libudev1

RUN addgroup --gid 1000 parity \
  && adduser --uid 1000 -gid 1000 --shell /bin/sh --disabled-password parity

USER parity

EXPOSE 8080 8545 8180

WORKDIR /home/parity

RUN mkdir -p /home/parity/.local/share/io.parity.ethereum/
COPY --chown=parity:parity --from=builder /parity/target/release/parity ./

ENTRYPOINT ["./parity"]
