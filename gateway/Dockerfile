FROM docker.io/library/alpine:edge AS builder

ENV RUST_TARGET "aarch64-unknown-linux-musl"

RUN apk add --no-cache curl gcc g++ musl-dev cmake make && \
    curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --component rust-src --default-toolchain nightly -y && \
    source $HOME/.cargo/env

WORKDIR /build

COPY ./gateway/Cargo.toml ./gateway/Cargo.lock ./

RUN mkdir src/
RUN echo 'fn main() {}' > ./src/main.rs

COPY ./models ../models

RUN source $HOME/.cargo/env && \
    cargo build --release --target="$RUST_TARGET"

RUN rm -f target/$RUST_TARGET/release/deps/gateway*

COPY ./models ../models
COPY ./gateway/src ./src

RUN source $HOME/.cargo/env && \
    cargo build --release --target="$RUST_TARGET" && \
    cp target/$RUST_TARGET/release/gateway /gateway && \
    strip /gateway

FROM scratch

COPY --from=builder /gateway /gateway

CMD ["./gateway"]
