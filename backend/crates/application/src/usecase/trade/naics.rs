use domain::valueobject::trade::{country::code::CountryCode, time::YearMonth, FieldPairs};
use kernel::core::{Pull, Push};
use macros::FieldKeyValuePairs;

use crate::infra::api::census::naics::model::{NaicsReq, NaicsRes};

pub struct NaicsUsecase<F, P> {
	pub fetcher: F,
	pub persister: P,
}

impl<F, P> NaicsUsecase<F, P>
where 
	F: Pull<NaicsReq, NaicsRes> + Send + Sync,
	P: Push<NaicsRes> + Send + Sync,
{
	pub fn new(fetcher: F, persister: P) -> Self {
		Self { fetcher, persister }
	}
}

#[derive(FieldKeyValuePairs)]
pub struct NaicsUsecaseInput {
    pub time: YearMonth,
    pub country_code: CountryCode,
}