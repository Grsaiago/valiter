use crate::Validator;

pub struct UppercaseStringValidator {}

impl UppercaseStringValidator {
    #[inline]
    pub fn new() -> UppercaseStringValidator {
        UppercaseStringValidator {}
    }
}

impl<T> Validator<T> for UppercaseStringValidator
where
    T: AsRef<str>,
{
    type ValidationError = ();

    fn validate(&mut self, value: &T) -> Result<(), Self::ValidationError> {
        let value: &str = value.as_ref();

        match value.matches(char::is_lowercase).count() != 0 {
            true => Err(()),
            false => Ok(()),
        }
    }
}
