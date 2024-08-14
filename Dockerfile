FROM rustlang/rust:nightly-bookworm AS build

WORKDIR /app

COPY . /app

RUN rustup --version
RUN rustup install nightly-2021-01-07 && \
    rustup default nightly-2021-01-07

RUN rustc --version && \
    rustup --version && \
    cargo --version

RUN apt-get update
RUN apt-get install -y libssl-dev default-libmysqlclient-dev

RUN cargo clean && cargo build --release
RUN strip ./target/release/raider

FROM debian:bookworm-slim

RUN apt-get update
RUN apt-get install -y openssl libmariadb3

WORKDIR /usr/src/raider

COPY ./res/assets/ ./res/assets/
COPY --from=build /app/target/release/raider /usr/local/bin/raider

CMD [ "raider", "-c", "/etc/raider.cfg" ]

EXPOSE 8080
