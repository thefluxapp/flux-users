FROM rust:alpine AS builder

WORKDIR /app

COPY . .
RUN apk --no-cache add musl-dev protoc
RUN cargo build --release

FROM nginx:alpine

WORKDIR /app

COPY --from=builder /app/target/release/flux-api /app/
COPY --from=builder /app/config/default.toml /app/config/
