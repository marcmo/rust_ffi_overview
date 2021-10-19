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

let x = NativeStruct { ... }

Transform ownership:
```rust
let ptr = Box::into_raw(Box::new(x));
// this box has to be freed by hand
```
Free memory:
```rust
let _ Box::from_raw(ptr);
// will be deallocated when out of scope
```
Borrowing:
```rust
let pointer: *const NativeStruct = &x;
```

### Sequences

```rust
vec![42u32, 43u32];
```
Borrowing:
```rust
let ptr = vec.as_ptr(); // Returns a raw pointer to the vector’s buffer
let len = vec.len();
```
Transfering ownership (moving)
```rust
let ptr = vec.as_mut_ptr();
let len = vec.len();
let cap = vec.capacity(); //needed by rust compiler to free when we get ownership back
mem::forget(vec); // transfer ownership outside, rust will not free the memory
```
Freeing memory:
```rust
#[no_mangle]```

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
