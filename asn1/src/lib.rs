#![allow(non_camel_case_types)]

// pub mod crate::types::*;
pub mod types;
pub mod bitstring;
pub mod duration;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
