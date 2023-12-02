# naga shader.wgsl shader.spv &&
# naga shader.spv shader-out.wgsl &&

build() {
    name=$1
    case $2 in
        "dynamic") opts="-dynamic -rdynamic" ;;
        *) opts="--export-table" ;;
    esac
    zig build-exe -target wasm32-wasi -O ReleaseSmall $opts \
        -I ../../include/wgpu-native \
        -I ../../include/wgpu-native/webgpu-headers \
        -I ../../include/taca \
        $name.zig && \
    wasm2wat --generate-names --fold-exprs --inline-exports --inline-imports \
        $name.wasm -o $name.wat && \
    wasm-opt -O4 -all $name.wasm -o $name.opt.wasm && \
    ls -l $name.opt.wasm && \
    wasm2wat --generate-names --fold-exprs --inline-exports --inline-imports \
        $name.opt.wasm -o $name.opt.wat
}

node minify.js && \
build explore-simple dynamic && \
build explore-webgpu
