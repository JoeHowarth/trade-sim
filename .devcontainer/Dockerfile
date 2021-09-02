# syntax = docker/dockerfile:1.2

#-------------------------------------------------------------------------------------------------------------
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
#-------------------------------------------------------------------------------------------------------------

FROM rust:stretch

# Or your actual UID, GID on Linux if not the default 1000
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

# # Setup env for rust
# ENV RUSTUP_HOME=/usr/local/rustup \
#     CARGO_HOME=/usr/local/cargo \
#     PATH=/usr/local/cargo/bin:$PATH


# Configure apt and install packages
RUN apt-get update \
    && apt-get -y install --no-install-recommends apt-utils dialog 2>&1 \
    #
    # Verify git, needed tools installed
    && apt-get -y install git procps lsb-release \
    #
    # Install other dependencies
    && apt-get install -y lldb-4.0 apt-transport-https ca-certificates 

RUN apt-get -y install software-properties-common gnupg gnupg1 gnupg2 && apt-get update

RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - \
    && apt-add-repository "deb http://apt.llvm.org/stretch/ llvm-toolchain-stretch main" \
    && apt-get update \
    && apt-get install -y clang clang-format clang-tidy lldb 

# Use clang as default compiler
RUN update-alternatives --install /usr/bin/cc cc /usr/bin/clang 90 \
    && update-alternatives --install /usr/bin/c++ c++ /usr/bin/clang++ 90 \
    && update-alternatives --install /usr/bin/cpp cpp /usr/bin/clang++ 90 

RUN apt-add-repository "deb http://deb.debian.org/debian stretch main" \
    && apt-get -y install libasound2-dev libudev-dev

# Install Rust components
# RUN rustup toolchain install beta \
#     && rustup default beta \
#     && rustup component add rls rust-analysis rust-src

# Install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup-init.sh \
    && chmod +x rustup-init.sh \
    && ./rustup-init.sh -y 

RUN wget -O sccache.tgz https://github.com/mozilla/sccache/releases/download/v0.2.15/sccache-v0.2.15-x86_64-unknown-linux-musl.tar.gz \
    && tar zxvf sccache.tgz \
    && chmod +x sccache-v0.2.15-x86_64-unknown-linux-musl/sccache \
    && mv sccache-v0.2.15-x86_64-unknown-linux-musl/sccache /usr/local/bin

RUN rustup default beta \ 
    && rustup component add rls rust-analysis rust-src 

ENV HOME=/home/root
ENV SCCACHE_CACHE_SIZE="5G"
ENV SCCACHE_DIR=$HOME/.cache/sccache
ENV RUSTC_WRAPPER="/usr/local/bin/sccache"

WORKDIR $HOME/code

RUN pwd \
    && cd $HOME/code \
    && git clone https://github.com/JoeHowarth/trade-sim.git \
    && cd trade-sim \
    && git checkout docker \
    && git pull \
    && cargo fetch

RUN --mount=type=cache,mode=0777,target=/home/root/.cache/sccache cd $HOME/code/trade-sim \
    && git pull \
    && cargo check \
    && cargo build

# Create a non-root user to use if preferred - see https://aka.ms/vscode-remote/containers/non-root-user.
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
    # [Optional] Add sudo support for non-root user
    && apt-get install -y sudo openssh-server \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME \
    && service ssh start

EXPOSE 22

RUN rm -f /etc/apt/apt.conf.d/docker-clean
    # # Clean up
    # && apt-get autoremove -y \
    # && apt-get clean -y \
    # && rm -rf /var/lib/apt/lists/*

CMD ["/usr/sbin/sshd","-D"]

# Switch back to dialog for any ad-hoc use of apt-get
ENV DEBIAN_FRONTEND=
