FROM rust:bookworm as builder

LABEL maintainer="Adrian.SOrtega@uclm.es"

ENV TAPO_USERNAME=defaultUsername
ENV TAPO_PASSWORD=defaultPassword
ENV MONGODB_URI=defaultUri
ENV USE_DOCKER=defaultDocker

WORKDIR /usr/src/tapo-main
COPY . .
RUN cargo build --release
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    openssl \
    arp-scan \
    && rm -rf /var/lib/apt/lists/*
    
COPY --from=builder /usr/src/tapo-main/target/release/tapo_mami /usr/local/bin/tapo_mami
CMD ["tapo_mami"]
