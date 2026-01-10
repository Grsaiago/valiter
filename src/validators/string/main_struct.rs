use crate::{
    And, Validator,
    base_structs::AlwaysValid,
    validators::string::{
        custom_fn::FnStringValidator, ends_with::EndsWithStringValidator,
        lowercase::LowercaseStringValidator, max::MaxLenStringValidator,
        min::MinLenStringValidator, starts_with::StartsWithStringValidator,
        uppercase::UppercaseStringValidator,
    },
};

pub struct StringValidator<V = AlwaysValid> {
    inner: V,
}

impl StringValidator<AlwaysValid> {
    #[inline]
    pub fn new() -> Self {
        Self { inner: AlwaysValid }
    }
}

impl<V, T> Validator<T> for StringValidator<V>
where
    T: AsRef<str>,
    V: for<'a> Validator<&'a str, ValidationError = ()>,
{
    type ValidationError = ();

    fn validate(&mut self, value: &T) -> Result<(), Self::ValidationError> {
        let s: &str = value.as_ref();
        self.inner.validate(&s)
    }
}

impl<V> StringValidator<V>
where
    V: for<'a> Validator<&'a str, ValidationError = ()>,
{
    pub fn min(self, min: usize) -> StringValidator<And<V, MinLenStringValidator>> {
        StringValidator {
            inner: And::new(self.inner, MinLenStringValidator::new(min)),
        }
    }

    pub fn max(self, max: usize) -> StringValidator<And<V, MaxLenStringValidator>> {
        StringValidator {
            inner: And::new(self.inner, MaxLenStringValidator::new(max)),
        }
    }

    pub fn lowercase(self) -> StringValidator<And<V, LowercaseStringValidator>> {
        StringValidator {
            inner: And::new(self.inner, LowercaseStringValidator {}),
        }
    }

    pub fn uppercase(self) -> StringValidator<And<V, UppercaseStringValidator>> {
        StringValidator {
            inner: And::new(self.inner, UppercaseStringValidator {}),
        }
    }

    pub fn starts_with<P: AsRef<str>>(
        self,
        pattern: P,
    ) -> StringValidator<And<V, StartsWithStringValidator>> {
        StringValidator {
            inner: And::new(self.inner, StartsWithStringValidator::new(pattern.as_ref())),
        }
    }

    pub fn ends_with<P: AsRef<str>>(
        self,
        pattern: P,
    ) -> StringValidator<And<V, EndsWithStringValidator>> {
        StringValidator {
            inner: And::new(self.inner, EndsWithStringValidator::new(pattern.as_ref())),
        }
    }

    pub fn with_func<F, P>(self, func: F) -> StringValidator<And<V, FnStringValidator<F>>>
    where
        F: Fn(&str) -> Result<(), ()>,
        P: AsRef<str>,
    {
        StringValidator {
            inner: And::new(self.inner, FnStringValidator::new(func)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_composable() {
        let test_string = "cinco".to_string();

        if let Err(()) = StringValidator::new().min(2).max(6).validate(&test_string) {
            panic!("failed to validate the max(10)")
        }
    }
}
