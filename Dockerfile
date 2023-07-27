ARG CPU_ARCH

FROM ghcr.io/goro-network/goro-builder-rust-llvm15:${CPU_ARCH} AS builder

ARG RUST_BUILD_ARG="-C target-cpu=generic"

WORKDIR /builder

COPY . .

RUN RUSTFLAGS="${RUST_BUILD_ARG}" cargo build --release

FROM ubuntu:22.04

LABEL org.opencontainers.image.authors "goro Developers <dev@goro.network>"
LABEL org.opencontainers.image.source "https://github.com/goro-network/goro-tx-indexer"
LABEL org.opencontainers.image.description "GORO Transaction Indexer"

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libc6 \
    libstdc++6 \
    libzstd1 && \
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /builder/target/release/goro-tx-indexer goro-tx-indexer

ENTRYPOINT [ "/app/goro-tx-indexer" ]
