# TODO Generalize some of this for container needs.
export PATH="$HOME/apps/wasi-sdk/bin:$PATH"
WASI_SYSROOT="$HOME/apps/wasi-sdk/share/wasi-sysroot"
export CRYSTAL_LIBRARY_PATH="$WASI_SYSROOT/lib/wasm32-wasi"
mkdir -p out
crystal build -o out/hi.wasm --target wasm32-unknown-wasi hi.cr \
    --link-flags="-L$HOME/apps/pcre2/install/lib"
