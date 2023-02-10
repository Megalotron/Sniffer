###
# Builder stage
# Install dependencies and build rust binary
###
FROM rust:alpine as builder

WORKDIR /app

# Copy sources
COPY . .

# Install dependencies
RUN apk add --no-cache musl-dev libpcap-dev

# Update dependencies
RUN cargo update

# Build application
RUN cargo build --release

###
# App stage
# Final application container
###
FROM alpine AS app

RUN apk add --no-cache musl-dev libpcap-dev

COPY --from=builder /app/target/release/sniffer /sniffer

ENTRYPOINT [ "/sniffer" ]

