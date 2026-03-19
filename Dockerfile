FROM rust:1.85.0-alpine3.21 AS builder

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/dashdb

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --no-default-features

FROM alpine:3.21.0

RUN addgroup -S dashgroup && adduser -S dashuser -G dashgroup
USER dashuser

WORKDIR /app

COPY --from=builder --chown=dashuser:dashgroup /usr/src/dashdb/target/release/dashdb ./dashdb

CMD ["./dashdb"]