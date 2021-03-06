# rasm
Wasm runtime written in Rust

For studying

## Play

### Compile WebAssembly

```
$ cat examples/myfunc.wat
(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add
  )
  (func $sub (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.sub
  )
  (func $mul (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.mul
  )
  (export "add" (func $add))
  (export "sub" (func $sub))
  (export "mul" (func $mul))
)
$ wat2wasm examples/myfunc.wat
```

### Call exported functions

```
$ cargo build --release
$ ./target/release/rasm myfunc.wasm add 5 6
11
$ ./target/release/rasm myfunc.wasm sub 5 6
-1
$ ./target/release/rasm myfunc.wasm mul 5 6
30
```
