use std::mem;

extern "C" {
    fn fill_array(ptr: *mut u8, len: usize, cap: usize) -> ();
}

#[no_mangle]
extern "C" fn vec_free(ptr: *mut u8, len: usize, cap: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, len, cap);
    }
}

pub fn test_vector() {
    let mut vec = vec![0u8, 1, 2];
    let ptr: *mut u8 = vec.as_mut_ptr();
    let len = vec.len();
    let cap = vec.capacity(); //needed by rust compiler to free when we get ownership back
    mem::forget(vec);
    unsafe {
        fill_array(ptr, len, cap);
    }
    println!("now free vector again");

    vec_free(ptr, len, cap);
}
