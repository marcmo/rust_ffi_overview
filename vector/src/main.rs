use std::mem;

extern "C" fn callback(ptr: *mut u8, len: usize, cap: usize) {
    println!("Rust: got back vector, free {} bytes of memory", cap);
    unsafe {
        let _ = Vec::from_raw_parts(ptr, len, cap);
    }
}

extern "C" {
    fn fill_array(ptr: *mut u8, len: usize, cap: usize);
    fn register_callback(cb: extern "C" fn(*mut u8, usize, usize)) -> i32;
}

fn main() {
    let mut vec = vec![0u8, 1, 2];
    let ptr: *mut u8 = vec.as_mut_ptr();
    let len = vec.len();
    let cap = vec.capacity(); //needed by rust compiler to free when we get ownership back
    mem::forget(vec);
    unsafe {
        register_callback(callback);
        fill_array(ptr, len, cap);
    }
}
