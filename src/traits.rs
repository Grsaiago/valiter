pub trait Validator<T> {
    type ValidationError;

    fn validate(&mut self, value: &T) -> Result<(), Self::ValidationError>;
}
