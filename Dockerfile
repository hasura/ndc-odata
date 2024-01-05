FROM rust:1.70.0-slim-buster AS build
WORKDIR app

RUN apt-get update && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      lld protobuf-compiler libssl-dev ssh git pkg-config

ENV RUSTFLAGS="-C link-arg=-fuse-ld=lld"
COPY . .

RUN cargo build --release --bin ndc-odata

FROM debian:buster-slim as ndc-odata

RUN apt-get update && \
  DEBIAN_FRONTEND=noninteractive apt-get install \
    --no-install-recommends --assume-yes curl

COPY --from=build /app/target/release/ndc-odata ./ndc-odata
CMD ["sh", "-c", "./ndc-odata serve --configuration metadata.json"]
