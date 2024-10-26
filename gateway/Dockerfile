FROM docker.io/library/alpine:edge AS builder

ENV RUST_TARGET "aarch64-unknown-linux-musl"

# Install necessary build tools
RUN apk add --no-cache curl gcc g++ musl-dev cmake make

# Install Rust
RUN curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --component rust-src --default-toolchain nightly -y

# Set working directory
WORKDIR /build

# Copy Cargo.toml and Cargo.lock to the working directory
COPY Cargo.toml Cargo.lock ./

# Create the src directory and main.rs file
RUN mkdir src/
RUN echo 'fn main() {}' > ./src/main.rs

# Build the project
RUN source $HOME/.cargo/env && \
    cargo build --release --target="$RUST_TARGET"

# Clean up previous binaries
RUN rm -f target/$RUST_TARGET/release/deps/gateway*

RUN echo "Listing files after creating src directory:" && ls -la ./

# Copy the source code (if there are more files in ./gateway/src)
COPY ./src ./src

# Build the project again with the full source
RUN source $HOME/.cargo/env && \
    cargo build --release --target="$RUST_TARGET" && \
    cp target/$RUST_TARGET/release/gateway /gateway && \
    strip /gateway

FROM scratch

# Copy the built binary from the builder stage
COPY --from=builder /gateway /gateway

# Set the command to run the application
CMD ["./gateway"]
