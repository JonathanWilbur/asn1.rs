#![allow(non_camel_case_types)]

pub mod types;
pub mod bitstring;
pub mod duration;
pub mod utctime;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
