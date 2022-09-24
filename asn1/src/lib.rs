#![allow(non_camel_case_types)]

pub mod types;
pub mod bitstring;
pub mod construction;
pub mod date;
pub mod duration;
pub mod error;
pub mod gentime;
pub mod tag;
pub mod time_of_day;
pub mod utctime;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
