use std::mem;

extern "C" {
    fn fill_array(ptr: *mut u8, len: usize, cap: usize);
}

fn main() {
    let mut vec = vec![0u8, 1, 2];
    let ptr: *mut u8 = vec.as_mut_ptr();
    let len = vec.len();
    let cap = vec.capacity(); //needed by rust compiler to free when we get ownership back
    mem::forget(vec);
    unsafe {
        fill_array(ptr, len, cap);
    }
    println!("now free vector again");
}
