use std::mem;

// Rust API we offer
extern "C" fn transfer_to_rust(ptr: *mut u8, len: usize, cap: usize) {
    println!("Rust: need to free {} bytes of memory", cap);
    unsafe {
        let v = Vec::from_raw_parts(ptr, len, cap);
        println!("Rust: got back vector {:?}", v);
    }
    println!("Rust: memory of vector freed");
}

// C-API we use
extern "C" {
    fn fill_array(ptr: *mut u8, len: usize, cap: usize);
    fn register_callback(cb: extern "C" fn(*mut u8, usize, usize)) -> i32;
}

fn main() {
    println!("Rust: prepare input vector");
    let (ptr, len, cap) = {
        let mut vec = vec![0u8, 1, 2];
        let ptr = vec.as_mut_ptr();
        let len = vec.len();
        let cap = vec.capacity(); //needed by rust compiler to free when we get ownership back
        mem::forget(vec);
        (ptr, len, cap)
    }; // vec goes out of scope...will NOT be deallocated
    unsafe {
        register_callback(transfer_to_rust);
        fill_array(ptr, len, cap); // transfer ownership
    }
}
