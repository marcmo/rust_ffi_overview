mod say;
use crate::say::sayit;

#[no_mangle]
pub extern "C" fn addition(a: u32, b: u32) -> u32 {
    sayit(a, b);
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
