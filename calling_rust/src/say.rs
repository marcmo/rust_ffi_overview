use ferris_says::say;
use std::io::{stdout, BufWriter};

pub fn sayit(a: u32, b: u32) {
    let greeting = format!("RUST adding {} and {}", a, b);
    let mut writer = BufWriter::new(stdout());
    say(greeting.as_bytes(), 24, &mut writer).unwrap();
}
