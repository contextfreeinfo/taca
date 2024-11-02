# Start

FROM ubuntu:22.04

ARG BINARYEN_VERSION=118
ARG C3_VERSION=0.6.2
ARG NAGA_VERSION=22.0.0
ARG NELUA_VERSION=ff7a42c275239933f6e615b2ad2e6a8d507afe7b
ARG NODE_VERSION=20.16.0
ARG RUST_VERSION=1.80.0
ARG VULKAN_SDK_VERSION=1.3.290.0
ARG WASI_SDK_VERSION=24.0
ARG WASM_PACK_VERSION=0.13.0
ARG ZIG_VERSION=0.13.0

# system dependencies and g++
RUN apt-get update && apt-get install -y \
    bash \
    build-essential \
    ca-certificates \
    curl \
    git \
    libxml2 \
    lz4 \
    vim \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Programming Languages

# c3
RUN curl -LO https://github.com/c3lang/c3c/releases/download/v${C3_VERSION}/c3-linux.tar.gz \
    && tar -xf c3-linux.tar.gz \
    && mv c3 /opt/c3
ENV PATH="/opt/c3:${PATH}"

# nelua
RUN git clone https://github.com/edubart/nelua-lang.git \
    && cd nelua-lang \
    && git checkout ${NELUA_VERSION} \
    && make \
    && make install \
    && cd .. \
    && rm -rf nelua-lang

# node
RUN curl -fsSL https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz \
    | tar -xJ --strip-components=1 -C /usr/local

# rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION}
ENV PATH="/root/.cargo/bin:${PATH}"

# wasi-sdk
RUN curl -LO https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-24/wasi-sdk-${WASI_SDK_VERSION}-x86_64-linux.tar.gz \
    && tar -xf wasi-sdk-${WASI_SDK_VERSION}-x86_64-linux.tar.gz \
    && mv wasi-sdk-${WASI_SDK_VERSION}-x86_64-linux /opt/wasi-sdk \
    && rm wasi-sdk-${WASI_SDK_VERSION}-x86_64-linux.tar.gz
ENV WASI_SDK="/opt/wasi-sdk"

# zig
RUN curl -LO https://ziglang.org/download/${ZIG_VERSION}/zig-linux-x86_64-${ZIG_VERSION}.tar.xz \
    && tar -xf zig-linux-x86_64-${ZIG_VERSION}.tar.xz \
    && mv zig-linux-x86_64-${ZIG_VERSION} /opt/zig \
    && rm zig-linux-x86_64-${ZIG_VERSION}.tar.xz
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
