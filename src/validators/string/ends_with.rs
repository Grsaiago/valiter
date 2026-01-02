use crate::Validator;

pub struct EndsWithStringValidator {
    pattern: String,
}

impl EndsWithStringValidator {
    pub fn new<P: AsRef<str>>(pattern: P) -> EndsWithStringValidator {
        EndsWithStringValidator {
            pattern: pattern.as_ref().to_string(),
        }
    }
}

impl<T> Validator<T> for EndsWithStringValidator
where
    T: AsRef<str>,
{
    type ValidationError = ();
    fn validate(&mut self, value: &T) -> Result<(), Self::ValidationError> {
        match value.as_ref().ends_with(&self.pattern) {
            true => Ok(()),
            false => Err(()),
        }
    }
}
