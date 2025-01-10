# Dockerfile
FROM rust:1.83.0-slim AS builder

WORKDIR /app
COPY . .
RUN apt-get update && apt-get install libpq5 -y
# Build your program for release
RUN cargo build --release

#Preduction stage
FROM debian:stable-slim
RUN apt-get update && apt-get install -y \
    libssl-dev \
    curl \
    wget
WORKDIR /user/local/bin

COPY --from=builder /app/target/release/apiusers /usr/local/bin/apiusers

CMD [ "apiusers" ]