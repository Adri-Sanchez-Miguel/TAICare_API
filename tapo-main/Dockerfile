FROM rust:latest as builder

LABEL maintainer="Adrian.SOrtega@uclm.es"

ENV TAPO_USERNAME=defaultUsername
ENV TAPO_PASSWORD=defaultPassword
ENV MONGODB_URI=mongodb://defaultUri

WORKDIR /usr/src/tapo-api
COPY . .
RUN cargo build --release --example tapo_p110_multiple

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    arp-scan \
    && rm -rf /var/lib/apt/lists/*
    
COPY --from=builder /usr/src/tapo-main/target/release/tapo-main /usr/local/bin/tapo-api
CMD ["./target/release/tapo_p110_multiple"]
