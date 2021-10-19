# Rust FFI Basics

* FFI: a way to talk to components written in other languages
* Based on platform-dependent C-Application Binary Interface (ABI)
  * defines how functions and data is accessed on a specific computing architecture -> low level,
    hardware dependent

## Types

* primitive/numeric types
* pointers

### Structures

Expose rust types to C:
* structs need to be re-structured:
```rust
pub struct Struct {
  v: Vec<u8>,
  name: String,
}
```
can be used in C/C++ in this form:
```rust
#[repr(C)]
pub struct NativeStruct {
  vec: *const u8,
  vec_len: usize,
  name: *const c_char,
}
```

### Borrowing and Ownership

Managing memory is more complicated than in pure rust

```rust
let x = NativeStruct { ... }
```

#### Transform ownership

```rust
// this box has to be freed by hand
let ptr = Box::into_raw(Box::new(x));
```
=> Free memory
```rust
// will be deallocated when out of scope
let _ = Box::from_raw(ptr);
```

#### Borrowing

```rust
// convert rust reference to pointer
let pointer: *const NativeStruct = &x;
```

### Sequences

see `vector` example

```rust
vec![42u32, 43u32];
```

## Using C code from Rust

### Calling Conventions

* A function's calling convention is part of its type
* The calling convention is specified next to extern
* Main calling conventions are "C" and "rust"

```rust
extern "C" {
    fn puts(s: *const c_char);
}
```

Every rust function is implicitly declared with extern "rust"

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
