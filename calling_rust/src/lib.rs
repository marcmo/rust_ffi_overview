use ferris_says::say;
use std::io::{ stdout, BufWriter };

#[no_mangle]
pub extern "C" fn addition(a: u32, b: u32) -> u32 {
    let greeting = format!("RUST adding {} and {}", a, b);
    // let out = b"Hello fellow Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout());
    say(greeting.as_bytes(), width, &mut writer).unwrap();
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = addition(2, 2);
        assert_eq!(result, 4);
    }
}
