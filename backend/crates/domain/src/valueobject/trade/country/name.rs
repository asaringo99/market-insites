use serde::Serialize;

use crate::{valueobject::{ApiColumn, Validatable, ValidationError, ValueObject}, Result};

const COUNTRY_NAME_LENGTH: usize = 50;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CountryName(String);

impl CountryName {
	pub fn as_value(&self) -> &str {
		&self.0.as_str()
	}
	pub fn to_string(&self) -> String {
		self.0.clone()
	}
}

impl ValueObject<String> for CountryName {
		fn new(v: impl Into<String>) -> anyhow::Result<Self> where Self: Sized {
			let v = Self(v.into());
			v.validate()?;
			Ok(v)
		}

		fn value(&self) -> String {
			self.0.clone()
		}
}

impl Validatable for CountryName {
	fn validate(&self) -> std::result::Result<(), crate::valueobject::ValidationError> {
			if self.0.len() != COUNTRY_NAME_LENGTH {
				Err(ValidationError::InvalidLength(format!("CountryName must be {} characters long", COUNTRY_NAME_LENGTH)))
			} else {
				Ok(())
			}
		}
}

impl ApiColumn for CountryName {
	const NAME: &'static str = "CTY_NAME";
}
