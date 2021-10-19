extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/lib.cpp")
        .cpp(true)
        .compile("example_lib.a");
}
