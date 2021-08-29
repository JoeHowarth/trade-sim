#-------------------------------------------------------------------------------------------------------------
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
#-------------------------------------------------------------------------------------------------------------

FROM buildpack-deps:stretch

# Or your actual UID, GID on Linux if not the default 1000
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

# Setup env for rust
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# Install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup-init.sh \
    && chmod +x rustup-init.sh \
    && ./rustup-init.sh -y --default-toolchain nightly-gnu

# Configure apt and install packages
RUN apt-get update \
    && apt-get -y install --no-install-recommends apt-utils dialog 2>&1 \
    #
    # Verify git, needed tools installed
    && apt-get -y install git procps lsb-release \
    #
    # Install other dependencies
    && apt-get install -y lldb-4.0 

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
RUN rustup toolchain install beta \
    && rustup default beta \
    && rustup component add rls rust-analysis rust-src 

    # Create a non-root user to use if preferred - see https://aka.ms/vscode-remote/containers/non-root-user.
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
    # [Optional] Add sudo support for non-root user
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME \
    #
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Switch back to dialog for any ad-hoc use of apt-get
ENV DEBIAN_FRONTEND=
