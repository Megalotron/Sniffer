###
# Builder stage
# Install dependencies and build rust binary
###
FROM rust:latest AS builder

WORKDIR /app
COPY . .

# Install dependencies
RUN apt update && apt install -y libpcap0.8-dev

# Build and install application
RUN cargo build --bin sniffer --release

###
# App stage
# Final application container
###
FROM debian:buster-slim

# Install dependencies
RUN apt update && apt install -y libpcap0.8

# Copy binary in final container
COPY --from=builder /app/target/release/sniffer /bin/sniffer

ENTRYPOINT ["sniffer"]
