use serde::{de::value, Serialize};

use crate::{valueobject::{ApiColumn, Validatable, ValidationError}, Result};

use super::{Naics, NAICS_2DIGIT_CODES};

const NAICS_CODE_LENGTH: usize = 2;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NaicsCode(String);

impl NaicsCode {
	pub fn new(value: String) -> Result<NaicsCode> {
		let value = Self(value);
		value.validate()?;
		Ok(value)
	}
	pub fn to_str(&self) -> &str {
		&self.0
	}
	pub fn to_string(&self) -> String {
		self.0.clone()
	}
}

impl Validatable for NaicsCode {
	fn validate(&self) -> std::result::Result<(), crate::valueobject::ValidationError> {
		if self.0.len() != NAICS_CODE_LENGTH {
			return Err(ValidationError::InvalidLength(format!("CountryName must be {} characters long", NAICS_CODE_LENGTH)));
		}
		Ok(())
	}
}

impl ApiColumn for NaicsCode {
	const NAME: &'static str = "NAICS";
}
