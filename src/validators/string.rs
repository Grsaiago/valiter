use crate::{And, Validator, base_structs::AlwaysValid};

pub struct StringValidator<V = AlwaysValid> {
    inner: V,
}
impl StringValidator<AlwaysValid> {
    pub fn new() -> Self {
        Self { inner: AlwaysValid }
    }
}
impl<V> StringValidator<V>
where
    V: Validator<String, ValidationError = ()>,
{
    pub fn min(self, min: usize) -> StringValidator<And<V, MinLenStringValidator>> {
        StringValidator {
            inner: And::new(self.inner, MinLenStringValidator::new(min)),
        }
    }

    pub fn max(self, min: usize) -> StringValidator<And<V, MaxLenStringValidator>> {
        StringValidator {
            inner: And::new(self.inner, MaxLenStringValidator::new(min)),
        }
    }
}
impl<V> Validator<String> for StringValidator<V>
where
    V: Validator<String, ValidationError = ()>,
{
    type ValidationError = ();

    fn validate(&mut self, value: &String) -> Result<(), Self::ValidationError> {
        self.inner.validate(value)
    }
}

pub struct MinLenStringValidator {
    len: usize,
}
impl MinLenStringValidator {
    #[inline]
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}
impl Validator<String> for MinLenStringValidator {
    type ValidationError = ();

    fn validate(&mut self, value: &String) -> Result<(), Self::ValidationError> {
        if value.len() < self.len {
            Err(())
        } else {
            Ok(())
        }
    }
}

pub struct MaxLenStringValidator {
    len: usize,
}
impl MaxLenStringValidator {
    #[inline]
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}
impl Validator<String> for MaxLenStringValidator {
    type ValidationError = ();

    fn validate(&mut self, value: &String) -> Result<(), Self::ValidationError> {
        if value.len() > self.len {
            Err(())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple_min() {
        let test_string = "cinco".to_string();

        if let Err(()) = StringValidator::new().min(2).validate(&test_string) {
            panic!("failed to validate the min(2)")
        }
    }

    #[test]
    fn err_simple_min() {
        let test_string = "cinco".to_string();

        if let Ok(()) = StringValidator::new().min(10).validate(&test_string) {
            panic!("should have failed on min(10)")
        }
    }

    #[test]
    fn ok_simple_max() {
        let test_string = "cinco".to_string();

        if let Err(()) = StringValidator::new().max(10).validate(&test_string) {
            panic!("failed to validate the max(10)")
        }
    }

    #[test]
    fn err_simple_max() {
        let test_string = "cinco".to_string();

        if let Ok(()) = StringValidator::new().max(3).validate(&test_string) {
            panic!("Should have failed on max(3)")
        }
    }
}
