use anyhow::Result;
use chrono::NaiveDate;
use serde::Serialize;

use crate::valueobject::{ApiColumn, Validatable, ValidationError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct YearMonth(String);

impl YearMonth {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let v = Self(value.into());
        v.validate()?;
        Ok(v)
    }
    pub fn as_str(&self) -> &str { &self.0 }
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Validatable for YearMonth {
    fn validate(&self) -> Result<(), ValidationError> {
        NaiveDate::parse_from_str(&format!("{}-01", self.0), "%Y-%m-%d")
            .map(|_| ())
            .map_err(|_| ValidationError::InvalidFormat(
                "YearMonth must be formatted as YYYY-MM".into()
            ))
    }
}

impl ApiColumn for YearMonth {
    const NAME: &'static str = "time";
}
