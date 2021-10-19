extern crate libc;

#[link(name = "snappy")]
extern "C" {
    fn snappy_max_compressed_length(source_length: libc::size_t) -> libc::size_t;
    fn triple_input(input: libc::c_int) -> libc::c_int;
}

#[repr(C)]
struct Data {
    a: u32,
    b: u16,
    c: u64,
}

fn main() {
    let x = unsafe { triple_input(100) };
    println!("triple 100: {}", x);
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("max compressed length of a 100 byte buffer: {}", x);
}
