use std::ops::Add;

use serde::Serialize;

use crate::{valueobject::{ApiColumn, Validatable}, Result};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MonthValue(u32);

impl MonthValue {
	pub fn new(value: u32) -> Result<MonthValue> {
		let value = Self(value);
		value.validate()?;
		Ok(value)
	}
	pub fn as_value(&self) -> u32 {
		self.0
	}
}

impl Add for MonthValue {
		type Output = Self;

		fn add(self, rhs: Self) -> Self::Output {
			Self(self.0 + rhs.0)
		}
}

impl Validatable for MonthValue {
	fn validate(&self) -> std::result::Result<(), crate::valueobject::ValidationError> {
			Ok(())
		}
}

impl ApiColumn for MonthValue {
	const NAME: &'static str = "ALL_VAL_MO";
}
