(module
  (type $t0 (func (param i32) (result i32)))
  (func $hi (import "env" "hi"))
  (func (export "run") (call $hi))
  (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
    local.get $p0
    i32.const 1
    i32.add
  )
)
