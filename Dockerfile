###
# Builder stage
# Install dependencies and build rust binary
###
FROM rust:alpine as builder

WORKDIR /app
COPY . .

# Install dependencies
RUN apk add --no-cache musl-dev libpcap-dev build-base

# Build and install application
RUN cargo build --release --bin sniffer

###
# App stage
# Final application container
###
FROM busybox AS app

COPY --from=builder /app/target/release/sniffer /bin/sniffer

ENTRYPOINT [ "sniffer" ]
