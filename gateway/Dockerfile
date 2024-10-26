FROM docker.io/library/alpine:edge AS builder

ENV RUST_TARGET "aarch64-unknown-linux-musl"

RUN apk upgrade
RUN apk add curl gcc g++ musl-dev cmake make
RUN curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --component rust-src --default-toolchain nightly -y

WORKDIR /build

COPY Cargo.toml Cargo.lock ./gateway/

RUN mkdir src/
RUN echo 'fn main() {}' > ./src/main.rs

RUN source $HOME/.cargo/env && \
        cargo build --release --target="$RUST_TARGET"

RUN rm -f target/$RUST_TARGET/release/deps/gateway*

COPY ./gateway/src ./src

RUN source $HOME/.cargo/env && \
    cargo build --release --target="$RUST_TARGET" && \
    cp target/$RUST_TARGET/release/gateway /gateway && \
    strip /gateway

FROM scratch

COPY --from=builder /gateway /gateway

CMD ["./gateway"]