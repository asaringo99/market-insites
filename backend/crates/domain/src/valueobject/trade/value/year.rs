use serde::Serialize;

use crate::{valueobject::{ApiColumn, Validatable, ValueObject}, Result};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct YearValue(u32);

impl ValueObject<u32> for YearValue {
	fn new(v: impl Into<u32>) -> anyhow::Result<Self> where Self: Sized {
		let v = Self(v.into());
		v.validate()?;
		Ok(v)
	}
	fn value(&self) -> u32 {
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
