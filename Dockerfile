# Build Service Node

FROM debian:buster-slim as snarkos-node-builder

RUN mkdir -p /app \
    && chown -R nobody:nogroup /app
WORKDIR /app

ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update && apt-get install -y build-essential curl clang gcc git libssl-dev llvm make pkg-config xz-utils

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

RUN git clone https://github.com/AleoHQ/snarkOS.git --depth 1 \
    && cd snarkOS \
    && cargo build --release \
    && mv ./target/release/snarkos /app/node-runner

# Build SnarkOS Rosetta Mentat
FROM debian:buster-slim as rosetta-mentat-builder

ARG BRANCH="main"

RUN mkdir -p /app \
    && chown -R nobody:nogroup /app
WORKDIR /app

ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update && apt-get install -y curl clang gcc git

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

RUN git clone -b $BRANCH https://github.com/monadicus/rosetta-snarkos.git \
    && cd rosetta-snarkos \
    && cargo build --profile release-docker \
    && mv ./target/release-docker/rosetta-snarkos /app/server \
    && mv ./docker.conf.toml /app/conf.toml

## Build Final Image
FROM debian:buster-slim

ENV ROCKET_ENV "production"

RUN apt-get update && apt-get install -y git

RUN mkdir -p /app \
    && chown -R nobody:nogroup /app \
    && mkdir -p /data \
    && chown -R nobody:nogroup /data

WORKDIR /app

# Copy binary from snarkos-node-builder
COPY --from=snarkos-node-builder /app/node-runner /app/node-runner

# Copy binary from rosetta-snarkos-server
COPY --from=rosetta-mentat-builder /app/* /app 

# Set permissions for everything added to /app
RUN chmod -R 755 /app/*

CMD ["/app/server", "./conf.toml"]
