use serde::Serialize;

use crate::{valueobject::{ApiColumn, Validatable, ValidationError, ValueObject}, Result};

const COUNTRY_CODE_LENGTH: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CountryCode(String);

impl CountryCode {
	pub fn to_string(&self) -> String {
		self.0.clone()
	}
}

impl ValueObject<String> for CountryCode {
	fn new(v: impl Into<String>) -> anyhow::Result<Self> where Self: Sized {
		let v = Self(v.into());
		v.validate()?;
		Ok(v)
	}

	fn value(&self) -> String {
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
