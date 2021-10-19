extern crate libc;

use libc::c_int;

extern "C" {
    fn triple_input(input: c_int) -> c_int;
}

fn triple(input: i32) -> i32 {
    unsafe { triple_input(input) }
}

fn main() {
    println!("triple 100: {}", triple(100));
}
