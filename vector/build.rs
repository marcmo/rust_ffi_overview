extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/lib.cpp")
        .cpp(true)
        .compile("vector_lib.a");
}
