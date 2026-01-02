use crate::Validator;

pub struct StartsWithStringValidator {
    pattern: String,
}

impl StartsWithStringValidator {
    pub fn new<P: AsRef<str>>(prefix: P) -> StartsWithStringValidator {
        StartsWithStringValidator {
            pattern: prefix.as_ref().to_string(),
        }
    }
}

impl<T> Validator<T> for StartsWithStringValidator
where
    T: AsRef<str>,
{
    type ValidationError = ();
    fn validate(&mut self, value: &T) -> Result<(), Self::ValidationError> {
        let value: &str = value.as_ref();

        match value.starts_with(self.pattern.as_str()) {
            true => Ok(()),
            false => Err(()),
        }
    }
}
