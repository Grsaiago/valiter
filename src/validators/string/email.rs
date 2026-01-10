use crate::Validator;
use regex::Regex;

pub struct EmailStringValidator {
    expression: Regex,
}
// impl EmailStringValidator {
//     #[inline]
//     pub fn new()
// }

impl<T> Validator<T> for EmailStringValidator
where
    T: AsRef<str>,
{
    type ValidationError = ();

    fn validate(&mut self, value: &T) -> Result<(), Self::ValidationError> {
        match self.expression.is_match(value.as_ref()) {
            true => Ok(()),
            false => Err(()),
        }
    }
}
