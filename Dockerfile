# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain

FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
#### Build dependencies
RUN apt-get update \
    && apt-get -y install --no-install-recommends apt-utils dialog 2>&1 \
    #
    # Verify git, needed tools installed
    && apt-get -y install git procps lsb-release \
    #
    # Install other dependencies
    && apt-get install -y apt-transport-https ca-certificates 

RUN apt-get -y install software-properties-common gnupg gnupg1 gnupg2 && apt-get update

RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - \
    && apt-add-repository "deb http://apt.llvm.org/buster/ llvm-toolchain-buster main" \
    && apt-get update \
    && apt-get install -y clang clang-format clang-tidy lldb 

# Use clang as default compiler
RUN update-alternatives --install /usr/bin/cc cc /usr/bin/clang 90 \
    && update-alternatives --install /usr/bin/c++ c++ /usr/bin/clang++ 90 \
    && update-alternatives --install /usr/bin/cpp cpp /usr/bin/clang++ 90 

RUN apt-add-repository "deb http://deb.debian.org/debian buster main" \
    && apt-get -y install libasound2-dev libudev-dev

WORKDIR /app
########

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies - this is the caching Docker layer!
FROM chef AS dependencies 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

# Build application
FROM chef AS builder
COPY --from=dependencies / /
COPY . .
RUN cargo install cargo-watch
RUN cargo build --bin trade-sim 
# CMD ["cargo", "run"]

# ENTRYPOINT [ "/app/start.sh" ]

### Need to build bevy with static linking instead of dylib ###
# We do not need the Rust toolchain to run the binary!
# FROM debian:buster-slim AS runtime
# WORKDIR /app
# # COPY --from=builder /app/target/debug /app
# COPY --from=builder /app/target/debug/trade-sim /usr/local/bin
# COPY --from=builder /app/target/debug/libbevy_dylib.so /usr/local/bin
# COPY --from=builder /app/target/debug/deps/libbevy_dylib-4a5935773de6d86d.so /usr/local/bin/deps

# ENTRYPOINT ["/usr/local/bin/trade-sim"]
