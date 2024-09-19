# First stage: build the server file.
FROM rust:alpine AS build

# Build dependencies
RUN apk add musl-dev
RUN apk add openssl-dev
RUN apk add protobuf

COPY . .
RUN cargo build --release --bin server

FROM alpine:latest AS exec

RUN apk add openssl-dev

COPY --from=build ./target/release/server /server/server

EXPOSE 5600/tcp
EXPOSE 6500/tcp

CMD ["/server/server"]