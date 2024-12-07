mkdir -p out && \
naga src/shader.wgsl out/shader.spv && \
nimble build -d:release
