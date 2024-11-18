# mkdir -p out && \
nimble build -d:release && \
lz4 -f9 out/textbox.wasm out/textbox.taca
