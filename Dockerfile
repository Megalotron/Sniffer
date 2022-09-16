FROM rust:alpine as builder

WORKDIR /app
COPY . .

# Install dependencies
RUN apk add --no-cache musl-dev libpcap-dev build-base

# Build and install application
RUN cargo install --path .

ENTRYPOINT [ "sniffer" ]
