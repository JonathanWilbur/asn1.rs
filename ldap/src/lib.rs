pub mod ldap;
pub use ldap::*;

pub fn asdf() {
    let id_oc: Vec<u32> = vec![];
    [id_oc, Vec::<u32>::from([30])].concat();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
