# Build Stage
FROM ubuntu:22.04 AS builder

## Install build dependencies.
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y cmake clang curl

RUN curl --proto =https --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
ENV PATH=/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /aho-corasick
WORKDIR /aho-corasick

RUN cd fuzz && cargo +nightly fuzz build find_one && \
    cargo +nightly fuzz build leftmost && \
    cargo +nightly fuzz build replacing && \
    cargo +nightly fuzz build searching && \
    cargo +nightly fuzz build searching_insensitive

# Package Stage
FROM ubuntu:22.04

COPY --from=builder /aho-corasick/fuzz/target/x86_64-unknown-linux-gnu/release/find_one /
COPY --from=builder /aho-corasick/fuzz/target/x86_64-unknown-linux-gnu/release/leftmost /
COPY --from=builder /aho-corasick/fuzz/target/x86_64-unknown-linux-gnu/release/replacing /
COPY --from=builder /aho-corasick/fuzz/target/x86_64-unknown-linux-gnu/release/searching /
COPY --from=builder /aho-corasick/fuzz/target/x86_64-unknown-linux-gnu/release/searching_insensitive /
