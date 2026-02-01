FROM rust:slim-trixie AS builder

RUN apt-get update && apt-get install -y \
  libssl-dev \
  pkg-config \
  libglib2.0-dev \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /src
RUN cargo new --bin spannweite
WORKDIR /src/spannweite
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release  # collects dependencies
RUN rm src/*.rs  # removes the `cargo new` generated files.

ADD . ./

RUN rm ./target/release/deps/spannweite*

RUN cargo build --release
RUN strip /src/spannweite/target/release/spannweite


FROM rust:slim-trixie AS build

ARG APP=/app

ENV TZ=Etc/UTC \
    RUST_LOG="spannweite=info"

RUN apt-get update && apt-get install -y \
  ca-certificates \
  tzdata \
  libglib2.0 \
  && rm -rf /var/lib/apt/lists/*


COPY --from=builder /src/spannweite/target/release/spannweite ${APP}/spannweite

WORKDIR ${APP}
STOPSIGNAL INT

ENTRYPOINT ["./spannweite"]