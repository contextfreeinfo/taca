import std/os

let wasi = getEnv("WASI_SDK")

switch("cpu", "wasm32")
switch("define", "noSignalHandler")
switch("define", "useMalloc")
switch("gc", "arc")
switch("gcc.exe", wasi / "bin" / "clang")
switch("gcc.linkerexe", wasi / "bin" / "clang")
# switch("gcc.options.linker", "-L" & wasi & "/share/wasi-sysroot/lib/wasm32-wasi -lwasi-emulated-mman -Wl,--allow-undefined")
switch("opt", "size")
switch("os", "any")
# switch("passC", "-flto --sysroot=" & (wasi / "share" / "wasi-sysroot"))
switch("passC", "--sysroot=" & (wasi / "share" / "wasi-sysroot"))
switch("passL", "-Wl,--no-entry,--strip-all,--gc-sections,--allow-undefined -mexec-model=reactor")
# switch("panics", "on")
switch("threads", "off")
