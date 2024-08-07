ARG RUST_VERSION=1.80.0
ARG APP_NAME=syn-ack

FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.3.0 AS xx

FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

COPY --from=xx / /

RUN apk add --no-cache clang lld musl-dev git file

ARG TARGETPLATFORM

RUN xx-apk add --no-cache musl-dev gcc

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=crates,target=crates \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=tests,target=tests \
    --mount=type=bind,source=test-utils,target=test-utils \
    --mount=type=cache,target=/app/target/,id=rust-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/git/db \
xx-cargo build --locked --release --target-dir ./target && \
cp ./target/$(xx-cargo --print-target-triple)/release/$APP_NAME /bin/server && \
xx-verify /bin/server

FROM alpine:3.18 AS final

LABEL org.opencontainers.image.source="https://github.com/pesca-dev/syn-ack"

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000

CMD ["/bin/server"]
