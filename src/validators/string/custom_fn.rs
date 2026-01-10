use crate::Validator;

pub struct FnStringValidator<F> {
    inner_func: F,
}

impl<F> FnStringValidator<F>
where
    F: Fn(&str) -> Result<(), ()>,
{
    pub fn new(func: F) -> FnStringValidator<F> {
        FnStringValidator { inner_func: func }
    }
}

impl<T, F> Validator<T> for FnStringValidator<F>
where
    T: AsRef<str>,
    F: Fn(&str) -> Result<(), ()>,
{
    type ValidationError = ();
    fn validate(&mut self, value: &T) -> Result<(), Self::ValidationError> {
        let value: &str = value.as_ref();
        (self.inner_func)(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StringValidator;

    #[test]
    fn simple_ok() {
        let test_string = "cinco".to_string();

        if let Err(()) = StringValidator::new()
            .with_func::<_, &str>(|_| Ok(()))
            .validate(&test_string)
        {
            panic!("the func should always return ok")
        }
    }

    #[test]
    fn simple_err() {
        let test_string = "cinco".to_string();

        if let Ok(()) = StringValidator::new()
            .with_func::<_, &str>(|_| Err(()))
            .validate(&test_string)
        {
            panic!("the func should always return err")
        }
    }
}
