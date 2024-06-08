(module
  (func $hi (import "" "hi"))
  (func (export "run") (call $hi))
)
