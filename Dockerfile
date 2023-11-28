FROM ghcr.io/bamboolabs-foundation/builder-rust-llvm:latest AS builder

ARG RUST_BUILD_ARG="-C target-cpu=generic"

WORKDIR /builder

COPY . .

RUN RUSTFLAGS="${RUST_BUILD_ARG}" cargo build --release

FROM ghcr.io/bamboolabs-foundation/base-ubuntu2204:latest

LABEL org.opencontainers.image.authors "nagara Network Developers <dev@nagara.network>"
LABEL org.opencontainers.image.source "https://github.com/nagara-network/tx-indexer"
LABEL org.opencontainers.image.description "nagara Network Transaction Indexer"

WORKDIR /app

COPY --from=builder /builder/target/release/nagara-tx-indexer nagara-tx-indexer

ENTRYPOINT [ "/app/nagara-tx-indexer" ]
