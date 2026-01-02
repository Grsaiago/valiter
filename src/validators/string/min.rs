use crate::Validator;

pub struct MinLenStringValidator {
    len: usize,
}

impl MinLenStringValidator {
    #[inline]
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}

// T is the type being validated
impl<T> Validator<T> for MinLenStringValidator
where
    T: AsRef<str>,
{
    type ValidationError = ();

    fn validate(&mut self, value: &T) -> Result<(), Self::ValidationError> {
        let s: &str = value.as_ref();
        match s.len() < self.len {
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
        let test_string = "cinco".to_string();

        if let Err(()) = StringValidator::new().min(2).validate(&test_string) {
            panic!("failed to validate the min(2)")
        }
    }

    #[test]
    fn simple_err() {
        let test_string = "cinco".to_string();

        if let Ok(()) = StringValidator::new().min(10).validate(&test_string) {
            panic!("should have failed on min(10)")
        }
    }
}
