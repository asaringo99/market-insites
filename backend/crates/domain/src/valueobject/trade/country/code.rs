use serde::Serialize;

use crate::{valueobject::{ApiColumn, Validatable, ValidationError}, Result};

const COUNTRY_CODE_LENGTH: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CountryCode(String);

impl CountryCode {
	pub fn new(value: impl Into<String>) -> Result<CountryCode> {
		let v = Self(value.into());
		v.validate()?;
		Ok(v)
	}
	pub fn as_value(&self) -> &str {
		&self.0.as_str()
	}
	pub fn to_string(&self) -> String {
		self.0.clone()
	}
}

impl Validatable for CountryCode {
	fn validate(&self) -> std::result::Result<(), crate::valueobject::ValidationError> {
			if self.0.len() != COUNTRY_CODE_LENGTH {
				Err(ValidationError::InvalidLength(format!("CountryCode must be {} characters long", COUNTRY_CODE_LENGTH)))
			} else {
				Ok(())
			}
		}
}

impl ApiColumn for CountryCode {
	const NAME: &'static str = "CTY_CODE";
}
