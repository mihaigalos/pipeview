FROM rust:alpine3.16 as base

COPY . /src

RUN apk update && \
    apk add --no-cache musl-dev

RUN rustup update 1.64 && rustup default 1.64

RUN cd /src && cargo build --release

FROM alpine:3.16 as tool

RUN apk update && apk add libgcc

COPY --from=base /src/target/release/pipeview /usr/local/bin

ENTRYPOINT [ "pipeview" ]
