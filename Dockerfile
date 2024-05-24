FROM rust:alpine3.19 as base

COPY . /src

RUN apk update && \
    apk add --no-cache musl-dev

RUN rustup update 1.74 && rustup default 1.74

RUN cd /src \
    &&  cargo build --release

FROM alpine:3.20 as tool

COPY --from=base /src/target/release/pipeview /usr/local/bin

ENTRYPOINT [ "pipeview" ]
CMD [ "--help" ]
