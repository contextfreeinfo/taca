import std/os

let wasi = getEnv("WASI_SDK")

switch("cpu", "wasm32")
switch("define", "noSignalHandler")
switch("define", "useMalloc")
switch("gc", "arc")
switch("gcc.exe", wasi / "bin" / "clang")
switch("gcc.linkerexe", wasi / "bin" / "clang")
switch("opt", "size")
switch("os", "any")
switch("passC", "--sysroot=" & (wasi / "share" / "wasi-sysroot"))
switch("passL", "-Wl,--no-entry,--strip-all,--gc-sections,--allow-undefined -mexec-model=reactor")
switch("threads", "off")
