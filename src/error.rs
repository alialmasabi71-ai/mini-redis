use core::fmt;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    EmptyValue,
    Custom(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyValue => write!(f, "value may not be empty"),
            ValidationError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for ValidationError {}

#[derive(Debug)]
pub enum StoreError {
    KeyNotFound,
    ValidationFailed(ValidationError),
    TypeMismatch { expected: &'static str, found: &'static str },
    LockError(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::KeyNotFound => write!(f, "key not found"),
            StoreError::ValidationFailed(err) => write!(f, "validation failed: {}", err),
            StoreError::TypeMismatch { expected, found } => {
                write!(f, "type mismatch: expected {}, found {}", expected, found)
            }
            StoreError::LockError(msg) => write!(f, "lock error: {}", msg),
        }
    }
}

impl Error for StoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            StoreError::ValidationFailed(err) => Some(err),
            _ => None,
        }
    }
}

impl From<ValidationError> for StoreError {
    fn from(err: ValidationError) -> Self {
        StoreError::ValidationFailed(err)
    }
}

impl<T> From<std::sync::PoisonError<T>> for StoreError {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        StoreError::LockError(err.to_string())
    }
}