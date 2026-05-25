FROM ghcr.io/cross-rs/aarch64-unknown-linux-gnu:main

ENV PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig:/usr/share/pkgconfig
ENV PKG_CONFIG_LIBDIR=/usr/lib/aarch64-linux-gnu/pkgconfig:/usr/share/pkgconfig
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV LIBCLANG_PATH=/usr/lib/llvm-18/lib

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        clang-18 \
        libclang-18-dev \
        pkg-config \
        python3 && \
    rm -rf /var/lib/apt/lists/*

# HailoRT runtime library and C headers (required for bindgen and linking)
COPY hailo_rs/aarch64_sysroot/lib/libhailort.so* /usr/aarch64-linux-gnu/lib/
COPY hailort_headers/ /usr/aarch64-linux-gnu/include/hailo/
COPY hailo/ /usr/include/hailo/