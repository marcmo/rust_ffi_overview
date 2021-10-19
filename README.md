# Rust FFI Basics

## Add some rust to your code

By default, any function you write in Rust will use the Rust ABI.
Instead, when building outwards facing FFI APIs we need to tell the compiler to use the system ABI.

Every function in your Rust-ffi API needs to have a corresponding header function.

```rust
#[no_mangle]
pub extern "C" fn rust_function() {}
```

would then become

```c
void rust_function();
```
