FROM rust:alpine AS builder

WORKDIR /app

COPY . .
RUN apk --no-cache add musl-dev protoc
RUN cargo build --release

FROM alpine

WORKDIR /app

COPY --from=builder /app/target/release/flux-users /app/
COPY --from=builder /app/settings/default.toml /app/
