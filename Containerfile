# Start

FROM ubuntu:22.04

ARG BINARYEN_VERSION=118
ARG NAGA_VERSION=22.0.0
ARG NODE_VERSION=20.16.0
ARG RUST_VERSION=1.80.0
ARG VULKAN_SDK_VERSION=1.3.290.0
ARG WASM_PACK_VERSION=0.13.0
ARG ZIG_VERSION=0.13.0

# system dependencies and g++
RUN apt-get update && apt-get install -y \
    bash \
    build-essential \
    ca-certificates \
    curl \
    lz4 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Programming Languages

# node
RUN curl -fsSL https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz \
    | tar -xJ --strip-components=1 -C /usr/local

# rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION}
ENV PATH="/root/.cargo/bin:${PATH}"

# zig
RUN curl -LO https://ziglang.org/download/${ZIG_VERSION}/zig-linux-x86_64-${ZIG_VERSION}.tar.xz \
    && tar -xf zig-linux-x86_64-${ZIG_VERSION}.tar.xz \
    && mv zig-linux-x86_64-${ZIG_VERSION} /opt/zig
ENV PATH="/opt/zig:${PATH}"

# Wasm & Shader Tools

# binaryen
RUN curl -LO https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
    && tar -xvzf binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
    && mv binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt /usr/local/bin/ \
    && rm -rf binaryen-version_${BINARYEN_VERSION}*

# naga
RUN cargo install naga-cli --version ${NAGA_VERSION}

# vulkan sdk
RUN curl -LO https://sdk.lunarg.com/sdk/download/${VULKAN_SDK_VERSION}/linux/vulkansdk-linux-x86_64-${VULKAN_SDK_VERSION}.tar.xz \
    && tar -xf vulkansdk-linux-x86_64-${VULKAN_SDK_VERSION}.tar.xz \
    && mv ${VULKAN_SDK_VERSION}/x86_64/bin/* /usr/bin/ \
    && mv ${VULKAN_SDK_VERSION}/x86_64/lib/* /usr/lib/ \
    && mv ${VULKAN_SDK_VERSION}/x86_64/include/* /usr/include/ \
    && mv ${VULKAN_SDK_VERSION}/x86_64/share/* /usr/share/ \
    && rm -rf vulkansdk-linux-x86_64-${VULKAN_SDK_VERSION}.tar.xz ${VULKAN_SDK_VERSION}

# wasm-pack
RUN npm install -g wasm-pack@${WASM_PACK_VERSION}

# Finish

# working directory
WORKDIR /workspace
