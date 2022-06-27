###
# Builder stage
# Install dependencies and build rust binary
###
FROM rust:alpine as builder

WORKDIR /app

# Copy sources
COPY . .

# Update dependencies
RUN cargo update

# Build application
RUN cargo build

###
# App stage
# Final application container
###
FROM busybox AS app

COPY --from=builder /app/target/debug/sniffer /bin/sniffer

ENTRYPOINT [ "sniffer" ]

