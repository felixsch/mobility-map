FROM rust:1.82 AS base
WORKDIR /build

RUN cargo install cargo-chef
RUN apt-get update && \
    apt-get install -y jq osm2pgsql vim curl wget && \
    rm -rf /var/lib/apt/lists/*

FROM base AS dependencies
COPY . .
RUN cargo chef prepare  --recipe-path dependencies.json

FROM base AS builder
WORKDIR /usr/src/mobility-map

COPY --from=dependencies /build/dependencies.json dependencies.json
RUN cargo chef cook --release --recipe-path dependencies.json

COPY . .
RUN cargo build --release --workspace
RUN ./scripts/install-all-applications
