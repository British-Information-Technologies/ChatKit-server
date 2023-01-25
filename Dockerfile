# First stage: build the server file.
FROM rust:alpine3.16 AS build
WORKDIR /app               # avoid the root directory
COPY ./ ./
RUN cargo build --release --bin server

# Second stage: actually run the server file.
FROM alpine:latest
WORKDIR /app
COPY --from=build /app/target/release/server ./server
CMD server