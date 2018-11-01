FROM rust:1.30-slim as base

WORKDIR /usr/src/kong-init
COPY . .

RUN DEBIAN_FRONTEND=noninteractive apt-get -q update && apt-get -yq install \
	pkg-config \
	libssl-dev \
	&& rm -rf /var/lib/apt/lists/* \
	&& cargo build --release

FROM debian:9.5-slim

RUN DEBIAN_FRONTEND=noninteractive apt-get -q update && apt-get -yq install \
	pkg-config \
	libssl-dev \
	&& rm -rf /var/lib/apt/lists/*

COPY --from=base /usr/src/kong-init/target/release/kong-init /usr/bin/kong-init

CMD /usr/bin/kong-init
