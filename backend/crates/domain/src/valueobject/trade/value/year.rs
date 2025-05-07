use serde::Serialize;

use crate::{valueobject::{ApiColumn, Validatable}, Result};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct YearValue(u32);

impl YearValue {
	pub fn new(value: u32) -> Result<YearValue> {
		let value = Self(value);
		value.validate()?;
		Ok(value)
	}
	pub fn as_value(&self) -> u32 {
		self.0
	}
}

impl Validatable for YearValue {
	fn validate(&self) -> std::result::Result<(), crate::valueobject::ValidationError> {
			Ok(())
		}
}

impl ApiColumn for YearValue {
	const NAME: &'static str = "ALL_VAL_YR";
}
