FROM rust:1.82.0
LABEL authors="eurel"

WORKDIR /usr/src/app
COPY . /usr/src/app

RUN cargo install --path .
RUN cargo build
