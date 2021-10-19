extern crate libc;

use libc::{c_int, size_t};

#[link(name = "snappy")]
extern "C" {
    /*
     * Returns the maximal size of the compressed representation of
     * input data that is "source_length" bytes in length.
     */
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
    /*
     * Takes the data stored in "input[0..input_length-1]" and stores
     * it in the array pointed to by "compressed".
     *
     * <compressed_length> signals the space available in "compressed".
     * If it is not at least equal to "snappy_max_compressed_length(input_length)",
     * SNAPPY_BUFFER_TOO_SMALL is returned. After successful compression,
     * <compressed_length> contains the true length of the compressed output,
     * and SNAPPY_OK is returned.
     */
    fn snappy_compress(
        input: *const u8,
        input_length: size_t,
        compressed: *mut u8,
        compressed_length: *mut size_t,
    ) -> c_int;
}

pub fn compress(src: &[u8]) -> Vec<u8> {
    unsafe {
        // get length and pointer to source
        let src_len = src.len() as size_t;
        let src_ptr = src.as_ptr();

        let mut dst_len = snappy_max_compressed_length(src_len);
        println!("max compressed len: {}", dst_len);
        // allocate memory for compressed data
        let mut dst = Vec::with_capacity(dst_len as usize);
        let dst_ptr = dst.as_mut_ptr();

        // call C-library
        snappy_compress(src_ptr, src_len, dst_ptr, &mut dst_len);
        // set length of actual data of result vector
        dst.set_len(dst_len as usize);
        dst
    }
}
fn main() {
    let text = "aaaaaaaaaaaaabbbbbbbbbbbbbcccccccccccc".to_owned();
    let uncompressed = text.as_bytes();
    let compressed = compress(uncompressed);
    println!(
        "compressed len={} vs. uncompressed len: {}) (compression ratio: {:.2}%)",
        compressed.len(),
        uncompressed.len(),
        (uncompressed.len() as f64 / compressed.len() as f64) * 100.0
    );
}
