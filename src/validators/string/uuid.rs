use std::str::FromStr;

use crate::Validator;
use regex::Regex;

pub struct UuidStringValidator {
    expression: Regex,
}

impl UuidStringValidator {
    pub fn new() -> Self {
        UuidStringValidator {
            // SAFETY: Regex is always valid
            expression: Regex::from_str(
                r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
            )
            .unwrap(),
        }
    }
}

impl<T> Validator<T> for UuidStringValidator
where
    T: AsRef<str>,
{
    type ValidationError = ();

    fn validate(&mut self, value: &T) -> Result<(), Self::ValidationError> {
        match self.expression.is_match(value.as_ref()) {
            true => Ok(()),
            false => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_ok() {
        const VALID_UUIDS: [&str; 10] = [
            "00000000-0000-0000-0000-000000000000", // Nil UUID
            "ffffffff-ffff-ffff-ffff-ffffffffffff", // Max UUID
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8", // UUID v1
            "6ba7b811-9dad-11d1-80b4-00c04fd430c8", // UUID v1
            "123e4567-e89b-12d3-a456-426614174000", // UUID v1
            "7d444840-9dc0-11d1-b245-5ffdce74fad2", // UUID v1
            "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", // UUID v4
            "3f6f9e64-0b4f-4c3a-9f3d-8e9c7b6a5d4c", // UUID v4
            "f47ac10b-58cc-4372-a567-0e02b2c3d479", // UUID v4
            "550e8400-e29b-41d4-a716-446655440000", // UUID v4
        ];

        let mut validator = UuidStringValidator::new();
        for uuid in VALID_UUIDS.iter() {
            assert!(
                validator.validate(uuid).is_ok(),
                "Expected {} to be valid",
                uuid
            );
        }
    }

    #[test]
    fn simple_err() {
        const INVALID_UUIDS: [&str; 15] = [
            "oieoie",                                     // Random string
            "",                                           // Empty string
            "550e8400-e29b-41d4-a716",                    // Too short
            "550e8400-e29b-41d4-a716-446655440000-extra", // Too long
            "550e8400e29b41d4a716446655440000",           // Missing hyphens
            "550e8400-e29b-41d4-a716-44665544000g",       // Invalid character (g)
            "550E8400-E29B-41D4-A716-446655440000",       // Uppercase (regex requires lowercase)
            "550e8400-e29b-41d4-a716-4466554400",         // Too few characters in last group
            "550e8400-e29b-41d4-a716-44665544000000",     // Too many characters in last group
            "550e8400-e29b-41d4-716-446655440000",        // Too few characters in middle group
            "550e8400-e29b-41d4a716-446655440000",        // Missing hyphen
            "550e8400-e29b-41d4-a716_446655440000",       // Wrong separator
            "g50e8400-e29b-41d4-a716-446655440000",       // Invalid character at start
            "550e8400-e29b-41d4-a716-44665544000 ",       // Trailing space
            " 550e8400-e29b-41d4-a716-446655440000",      // Leading space
        ];

        let mut validator = UuidStringValidator::new();
        for uuid in INVALID_UUIDS.iter() {
            assert!(
                validator.validate(uuid).is_err(),
                "Expected {} to be invalid",
                uuid
            );
        }
    }
}
