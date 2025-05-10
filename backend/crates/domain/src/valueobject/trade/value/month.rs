use std::ops::Add;

use serde::Serialize;

use crate::{valueobject::{ApiColumn, Validatable, ValueObject}, Result};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MonthValue(u32);

impl ValueObject<u32> for MonthValue {
	fn new(v: impl Into<u32>) -> anyhow::Result<Self> where Self: Sized {
		let v = Self(v.into());
		v.validate()?;
		Ok(v)
	}
	fn value(&self) -> u32 {
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
