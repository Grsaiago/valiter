use crate::Validator;
use regex::Regex;

pub struct EmailStringValidator {
    expression: Regex,
}

impl EmailStringValidator {
    #[inline]
    pub fn new() -> Self {
        // SAFETY: The regex is valid xD
        Self {
            expression: Regex::new(
                r"/^[a-zA-Z0-9.!#$%&'*+\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/",
            ).unwrap(),
        }
    }
}

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
