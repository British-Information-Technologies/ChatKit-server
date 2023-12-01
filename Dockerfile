# First stage: build the server file.
FROM rust:alpine AS build

RUN apk add musl-dev


RUN apk upgrade --update-cache --available && \
    apk add openssl-dev && \
    rm -rf /var/cache/apk/*

COPY . .
CMD ["cargo", "run", "--release", "--bin", "server"]
