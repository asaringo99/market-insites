use domain::{entity::trade::naics::Naics, valueobject::trade::{country::{code::CountryCode, name::CountryName}, time::YearMonth, value::{month::MonthValue, year::YearValue}, FieldPairs}};
use kernel::core::{Pull, Push};
use macros::{FieldKeyValuePairs, RequestSpec, ResponseSpec};

use crate::infra::api::census::naics::model::{NaicsReq, NaicsRes};

pub struct NaicsUsecase<F, P> {
	pub fetcher: F,
	pub persister: P,
}

impl<F, P> NaicsUsecase<F, P>
where 
	F: Pull<NaicsUsecaseInput, NaicsUsecaseFetched> + Send + Sync,
	P: Push<NaicsUsecaseOutput> + Send + Sync,
{
	pub fn new(fetcher: F, persister: P) -> Self {
		Self { fetcher, persister }
	}
}

#[derive(FieldKeyValuePairs, RequestSpec)]
pub struct NaicsUsecaseInput {
    pub time: YearMonth,
    pub country_code: CountryCode,
}

#[derive(ResponseSpec)]

pub struct NaicsUsecaseFetched {
	pub country_code:  CountryCode, // CTY_CODE
	pub country_name:  CountryName, // CTY_NAME
	pub naics:         Naics, // NAICS
	pub month_value:   MonthValue, // ALL_VAL_MO
	pub year_value:    YearValue, // ALL_VAL_YR
	pub time:          YearMonth, // time (YYYY-MM)
	// pub naics_desc:    String, // NAICS_SDESC
}
pub struct NaicsUsecaseOutput {
	
}