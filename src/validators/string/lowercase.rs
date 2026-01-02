use crate::Validator;

pub struct LowercaseStringValidator {}

impl<T> Validator<T> for LowercaseStringValidator
where
    T: AsRef<str>,
{
    type ValidationError = ();

    fn validate(&mut self, value: &T) -> Result<(), Self::ValidationError> {
        let value: &str = value.as_ref();

        match value.matches(char::is_uppercase).count() != 0 {
            true => Err(()),
            false => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StringValidator;

    #[test]
    fn simple_ok() {
        let test_string = "test".to_string();

        if let Err(()) = StringValidator::new().lowercase().validate(&test_string) {
            panic!("failed to validate the max(10)")
        }
    }

    #[test]
    fn simple_err() {
        let test_string = "Test".to_string();

        if let Ok(()) = StringValidator::new().lowercase().validate(&test_string) {
            panic!("Should have failed on max(3)")
        }
    }
}
