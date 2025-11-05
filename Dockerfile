FROM rust:alpine3.19 as base

COPY . /src

RUN apk update && \
    apk add --no-cache musl-dev

RUN cd /src \
    && cargo build --release

FROM alpine:3.22 as tool

COPY --from=base /src/target/release/pipeview /usr/local/bin

ENTRYPOINT [ "pipeview" ]
CMD [ "--help" ]
