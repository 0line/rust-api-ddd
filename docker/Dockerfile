FROM rust:1.80.1-slim AS builder
WORKDIR /apirust
COPY . .
RUN cargo install --path .
#CMD ["./target/release/apirust"]

FROM debian:buster-slim
RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*
#COPY --from=builder /usr/apirust /usr/local/cargo/bin/apirust

COPY --from=builder --chown=app_user:app_user /apirust/target/release/apiusers .

RUN adduser -D app_user

USER app_user
CMD ["./apirust"]
