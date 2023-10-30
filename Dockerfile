ARG CACHE_VERSION=v1
ARG ARCH=amd64
ARG RUST_CACHE_ID=com.github.naftulikay.lambda-ecr-rewrite.rust.${ARCH}.${CACHE_VERSION}
ARG RUST_BIN=lambda

FROM rust:alpine as builder

# import
ARG RUST_BIN
ARG RUST_CACHE_ID
ARG RUST_TARGET

ARG BUILDPLATFORM
ARG TARGETPLATFORM
RUN echo "*** building on ${BUILDPLATFORM} for ${TARGETPLATFORM}..."

RUN apk add --no-cache openssl libc-dev

RUN install -d /usr/src/app /usr/src/app/build
WORKDIR /usr/src/app

COPY ./Cargo.toml ./Cargo.lock ./

RUN --mount=type=cache,target=/usr/src/app/target,id=${RUST_CACHE_ID} \
    cargo fetch --locked

COPY ./src/ ./src/

RUN --mount=type=cache,target=/usr/src/app/target,id=${RUST_CACHE_ID} \
    cargo build --release --bin ${RUST_BIN} && \
    cp "$(find target/ -mindepth 1 -type f -path '**/release/*' -name "${RUST_BIN}")" build/ && \
    strip build/${RUST_BIN}

FROM alpine:latest

ARG BUILDPLATFORM
ARG TARGETPLATFORM

RUN echo "*** building on ${BUILDPLATFORM} for ${TARGETPLATFORM}..."

# create lambda user and group
RUN addgroup -g 1000 lambda && adduser -D -H -h /usr/src/app -G lambda -u 1000 lambda
RUN install -d -o lambda -g lambda /usr/src/app && chown -R lambda:lambda /usr/src/app

USER lambda
WORKDIR /usr/src/app

# install binary
COPY --from=builder --chown=lambda:lambda /usr/src/app/build/lambda ./

# integration test
RUN /usr/src/app/lambda test

ENTRYPOINT ["/usr/src/app/lambda"]


