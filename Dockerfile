FROM rust:1.82 AS base
WORKDIR /build

RUN cargo install cargo-chef

ENV NODE_VERSION=v23.10.0
ENV YARN_VERSION=4.5.3

RUN apt-get update && apt-get install -y jq osm2pgsql vim curl wget git && \
    rm -rf /var/lib/apt/lists/*

RUN curl -o /tmp/node.tar.xz https://nodejs.org/dist/${NODE_VERSION}/node-${NODE_VERSION}-linux-x64.tar.xz
RUN tar -xf /tmp/node.tar.xz -C /usr/local --strip-components=1

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
RUN ./scripts/build-assets
