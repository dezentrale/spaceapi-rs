FROM --platform=linux/x86_64 docker.io/library/rust:1.72 AS builder
ARG TARGET=x86_64-unknown-linux-musl
ARG BUILD_TYPE=release
ENV DEBCONF_FRONTEND=noninteractive
RUN apt-get update \
    && apt-get install --yes \
        gcc-aarch64-linux-gnu \
        gcc-arm-linux-gnueabihf \
        musl-tools
COPY . /usr/src
WORKDIR /usr/src
RUN \
    if [ "${BUILD_TYPE}" = "release" ] ; then \
        cargo build --bins --target "${TARGET}" --release; \
    else \
        cargo build --bins --target "${TARGET}" --release; \
    fi

FROM scratch
LABEL org.opencontainers.image.source="https://github.com/dezentrale/spaceapi-rs"
LABEL org.opencontainers.image.vendor="dezentrale"
LABEL org.opencontainers.image.base.name="scratch"
ARG BINARY=target/x86_64-unknown-linux-musl/release/spaceapi-dezentrale-server
ENV RUST_LOG=WARN
COPY --from=builder /usr/src/$BINARY /spaceapi
ENTRYPOINT ["/spaceapi"]
