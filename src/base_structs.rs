use crate::Validator;

pub struct AlwaysValid;
impl<T> Validator<T> for AlwaysValid {
    type ValidationError = ();
    fn validate(&mut self, _value: &T) -> Result<(), Self::ValidationError> {
        Ok(())
    }
}

/// The 'glue' to chain validators
pub struct And<A, B> {
    first: A,
    second: B,
}

impl<A, B> And<A, B> {
    #[inline]
    pub fn new(first: A, second: B) -> Self {
        And { first, second }
    }
}

impl<T, A, B> Validator<T> for And<A, B>
where
    A: Validator<T, ValidationError = ()>,
    B: Validator<T, ValidationError = A::ValidationError>,
{
    type ValidationError = ();

    fn validate(&mut self, value: &T) -> Result<(), Self::ValidationError> {
        self.first.validate(value)?;
        self.second.validate(value)?;
        Ok(())
    }
}
