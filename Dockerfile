FROM rust:1.62.0-slim-bullseye AS builder

WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \
    set -eux; \
    rustup install stable; \
    cargo build --release; \
    cp -a target/release/wf /app/wf

################################################################################
FROM debian:11.3-slim

RUN set -eux; \
    export DEBIAN_FRONTEND=noninteractive; \
    apt update; \
    apt install --yes --no-install-recommends bind9-dnsutils iputils-ping iproute2 curl ca-certificates htop; \
    apt clean autoclean; \
    apt autoremove --yes; \
    rm -rf /var/lib/{apt,dpkg,cache,log}/; \
    echo "Installed base utils!"

WORKDIR /app

COPY --from=builder /app/wf .
COPY build build
EXPOSE 3000
ENTRYPOINT ["/app/wf", "server"]
CMD ["-a", "0.0.0.0:3000"]
