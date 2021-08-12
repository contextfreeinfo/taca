```
(cd examples && zig build-exe hello.zig -target wasm32-freestanding -dynamic) \
&& zig build-exe -Ivendor/wasmtime/include -Lvendor/wasmtime/lib src/tacana.zig -lwasmtime -lc \
&& ./tacana
```
