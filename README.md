# Taca

A runtime for multimedia wasm apps that runs native and in browsers.

Build optimized:

```
# TODO Minify js.
cargo build --target wasm32-unknown-unknown --profile release-lto
wasm-opt -Os target/wasm32-unknown-unknown/release-lto/taca.wasm \
    -o target/wasm32-unknown-unknown/release-lto/taca.opt.wasm
```

These files are copied and maybe modified from miniquad:

- examples/gl.js
- examples/index.html

## Wasmer

```
   Compiling wasmer v4.3.1
   Compiling taca v0.1.0 (/home/tom/projects/taca)
    Finished `release` profile [optimized] target(s) in 1m 22s
     Running `target/release/taca`
2024-06-08T03:21:36Z tom@hierba:~/projects/taca
$ ls -lh target/release
total 11M
drwxr-xr-x 109 tom tom 4.0K Jun  7 20:20 build
drwxr-xr-x   2 tom tom  64K Jun  7 20:21 deps
drwxr-xr-x   2 tom tom 4.0K Jun  6 19:41 examples
drwxr-xr-x   2 tom tom 4.0K Jun  6 19:41 incremental
-rwxr-xr-x   2 tom tom  11M Jun  7 20:21 taca
-rw-r--r--   1 tom tom  198 Jun  7 20:21 taca.d
2024-06-08T03:27:14Z tom@hierba:~/projects/taca
...
   Compiling taca v0.1.0 (/home/tom/projects/taca)
    Finished `release-lto` profile [optimized] target(s) in 1m 40s
     Running `target/release-lto/taca`
2024-06-08T03:29:33Z tom@hierba:~/projects/taca
$ ls -lh target/release-lto/
total 5.1M
drwxr-xr-x 106 tom tom 4.0K Jun  7 20:27 build
drwxr-xr-x   2 tom tom  64K Jun  7 20:29 deps
drwxr-xr-x   2 tom tom 4.0K Jun  7 05:35 examples
drwxr-xr-x   2 tom tom 4.0K Jun  7 05:35 incremental
-rwxr-xr-x   2 tom tom 5.0M Jun  7 20:29 taca
-rw-r--r--   1 tom tom  202 Jun  7 20:29 taca.d
2024-06-08T03:29:39Z tom@hierba:~/projects/taca
```

## Wasmtime

```
   Compiling taca v0.1.0 (/home/tom/projects/taca)
    Finished `release` profile [optimized] target(s) in 1m 32s
     Running `target/release/taca`
...
$ ls -lh target/release
total 13M
drwxr-xr-x 71 tom tom 4.0K Jun  7 19:54 build
drwxr-xr-x  2 tom tom  40K Jun  7 20:07 deps
drwxr-xr-x  2 tom tom 4.0K Jun  6 19:41 examples
drwxr-xr-x  2 tom tom 4.0K Jun  6 19:41 incremental
-rwxr-xr-x  2 tom tom  13M Jun  7 20:07 taca
-rw-r--r--  1 tom tom  233 Jun  7 20:02 taca.d
...
   Compiling taca v0.1.0 (/home/tom/projects/taca)
    Finished `release-lto` profile [optimized] target(s) in 1m 58s
     Running `target/release-lto/taca`
Compiling module...
Initializing...
Creating callback...
Instantiating module...
Extracting export...
Calling export...
Calling back...
> hello, world!
Done.
2024-06-08T03:09:40Z tom@hierba:~/projects/taca
$ ls -lh target/release-lto/
total 6.0M
drwxr-xr-x 68 tom tom 4.0K Jun  7 20:07 build
drwxr-xr-x  2 tom tom  40K Jun  7 20:09 deps
drwxr-xr-x  2 tom tom 4.0K Jun  7 05:35 examples
drwxr-xr-x  2 tom tom 4.0K Jun  7 05:35 incremental
-rwxr-xr-x  2 tom tom 5.9M Jun  7 20:09 taca
-rw-r--r--  1 tom tom  237 Jun  7 20:09 taca.d
2024-06-08T03:09:44Z tom@hierba:~/projects/taca
```
