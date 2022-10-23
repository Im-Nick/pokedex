FROM rust:1.64-slim-buster as Builder

RUN apt-get update && apt-get install -y musl-tools musl-dev cmake pkg-config libssl-dev
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/app

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/pokedex

FROM alpine:latest

RUN addgroup -g 1000 user

RUN adduser -D -s /bin/sh -u 1000 -G user user

WORKDIR /home/user/bin/

RUN apk add --no-cache \
    perl \
    wget \
    openssl \
    ca-certificates \
    libc6-compat \
    libstdc++

# copy the binary into the final image
COPY --from=builder /usr/app/target/x86_64-unknown-linux-musl/release/pokedex .

RUN chown user:user pokedex

USER user

CMD ["./pokedex"]
