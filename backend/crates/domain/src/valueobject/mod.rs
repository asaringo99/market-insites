use std::result::Result;

pub mod trade;

#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidLength(String),
		InvalidFormat(String),
}

impl std::fmt::Display for ValidationError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ValidationError::InvalidLength(message) => write!(f, "invalid length error. {}", message),
			ValidationError::InvalidFormat(message) => write!(f, "invalid format error. {}", message),
		}
	}
}

impl std::error::Error for ValidationError {}

pub trait Validatable {
	fn validate(&self) -> Result<(), ValidationError>;
}

pub trait ValueObject<T: Into<T>> {
	fn new(v: impl Into<T>) -> anyhow::Result<Self> where Self: Sized;
	fn value(&self) -> T;
}

pub trait ApiColumn: serde::Serialize {
	const NAME: &'static str;
}