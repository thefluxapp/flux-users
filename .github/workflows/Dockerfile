FROM rust:alpine AS builder

WORKDIR /app

COPY . .
RUN apk --no-cache add musl-dev protoc protobuf-dev
RUN cargo build --release -p flux-users -p flux-users-migration

FROM alpine

WORKDIR /app

COPY --from=builder /app/target/release/flux-users /app/
COPY --from=builder /app/target/release/flux-users-migration /app/
COPY --from=builder /app/settings/default.toml /app/
