(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func))
  (import "env" "taca_windowSetTitle" (func (;0;) (type 0)))
  (import "env" "taca_draw" (func (;1;) (type 1)))
  (import "wasi_snapshot_preview1" "proc_exit" (func (;2;) (type 0)))
  (func (;3;) (type 2)
    i32.const 16777288
    call 0
    i32.const 16777300
    i32.const 16777308
    call 1)
  (func (;4;) (type 1) (param i32 i32) ;; params 0 (in) and 1 (out)
    local.get 1 ;; get param 1
    i32.const 8 ;; constant 8
    i32.add ;; param 1 + 8
    i32.const 0 ;; constant 0
    i64.load offset=16777232 align=4 ;; load 8 bytes from address 16777232
    i64.store align=4 ;; store those bytes at param 1 + 8
    local.get 1 ;; get param 1
    i32.const 0 ;; constant 0
    i64.load offset=16777224 align=4 ;; load 8 bytes from address 16777224
    i64.store align=4) ;; store those bytes at param 1
  (func (;5;) (type 1) (param i32 i32) ;; params 0 (in) and 1 (out)
    (local f32) ;; declare local 2
    local.get 1 ;; get parameter 1 (address)
    local.get 0 ;; get parameter 0 (address)
    i32.load offset=4 ;; load i32 from param 0
    i32.load ;; load i32 address from wherever that was
    local.tee 0 ;; set param 0 to whatever was loaded and keep here
    f32.load ;; load from that address
    f32.store ;; store whatever was loaded into param 1 address
    local.get 0 ;; get whatever we stored in param 0
    f32.load offset=4 ;; load from 4 bytes after that address
    local.set 2 ;; set local 2 to whatever we just loaded
    local.get 1 ;; get param 1 (address)
    i64.const 4575657221408423936 ;; f32le (0.0, 1.0)
    i64.store offset=8 align=4 ;; store those values at offset 8 from param 1
    local.get 1 ;; get param 1 (address)
    local.get 2 ;; get what we set in local 2 (4 bytes from param 1 indirection)
    f32.store offset=4) ;; store that at offset 4 from param 1
  (func (;6;) (type 2)
    call 3
    i32.const 0
    call 2
    unreachable)
  (table (;0;) 3 3 funcref)
  (memory (;0;) 257)
  (global (;0;) (mut i32) (i32.const 16777216))
  (export "memory" (memory 0))
  (export "_start" (func 6))
  (export "__indirect_function_table" (table 0))
  (elem (;0;) (i32.const 1) func 4 5)
  (data (;0;) (i32.const 16777216) "\18\00\00\01\08\00\00\00\00\00\00\00\00\00\00?\00\00\80?\00\00\80?\00\00\00\00\00\00\00\00\00\00\80?\00\00\00\00\00\00\00\00\00\00\80?\00\00\00\00\00\00\80?\00\00\80?\00\00\80?\00\00\80?\00\00\00\00")
  (data (;1;) (i32.const 16777288) "Hi, Taca!\00\00\00\01\00\00\00\02\00\00\00\00\00\00\00\00\00\00\01\02\00\00\00\06\00\00\00\10\00\00\00"))
