FROM rust:alpine3.17 as base

COPY . /src

RUN apk update && \
    apk add --no-cache musl-dev

RUN rustup update 1.70 && rustup default 1.70

RUN cd /src \
    &&  cargo build --release

FROM alpine:3.20 as tool

COPY --from=base /src/target/release/pipeview /usr/local/bin

ENTRYPOINT [ "pipeview" ]
CMD [ "--help" ]
