FROM rust:alpine3.17 as base

COPY . /src

RUN rustup update 1.64 && rustup default 1.64

RUN cd /src \
    &&  cargo build --release

FROM alpine:3.17 as tool

COPY --from=base /src/target/release/pipeview /usr/local/bin

ENTRYPOINT [ "pipeview" ]
CMD [ "--help" ]
