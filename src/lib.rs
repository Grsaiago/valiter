pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod base_structs;
mod traits;
mod validators;

pub use base_structs::And;
pub use traits::Validator;
pub use validators::StringValidator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
