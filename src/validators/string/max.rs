use crate::Validator;

pub struct MaxLenStringValidator {
    len: usize,
}

impl MaxLenStringValidator {
    #[inline]
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}

// T is the type being validated
impl<T> Validator<T> for MaxLenStringValidator
where
    T: AsRef<str>,
{
    type ValidationError = ();

    fn validate(&mut self, value: &T) -> Result<(), Self::ValidationError> {
        let s: &str = value.as_ref();
        if s.len() > self.len { Err(()) } else { Ok(()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StringValidator;

    #[test]
    fn simple_ok() {
        let test_string = "cinco".to_string();

        if let Err(()) = StringValidator::new().max(10).validate(&test_string) {
            panic!("failed to validate the max(10)")
        }
    }

    #[test]
    fn simple_err() {
        let test_string = "cinco".to_string();

        if let Ok(()) = StringValidator::new().max(3).validate(&test_string) {
            panic!("Should have failed on max(3)")
        }
    }
}
