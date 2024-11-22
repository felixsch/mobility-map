FROM rust:1.82

RUN apt-get update && \
    apt-get install -y jq && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/mobility-map

COPY . .

RUN cargo fetch
RUN cargo build --release

RUN ./scripts/install-all-packages
