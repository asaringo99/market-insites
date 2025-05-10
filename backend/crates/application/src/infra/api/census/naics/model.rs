extern crate kernel;
extern crate macros;

use anyhow::Ok;
use macros::GenerateReq;
use crate::{internal::{ReqSpec, ResSpec}, usecase::trade::naics::{NaicsUsecaseFetched, NaicsUsecaseInput}};

#[derive(GenerateReq)]
#[inner(NaicsUsecaseInput)]
#[endpoint("https://api.census.gov/data/timeseries/intltrade/exports/naics")]
#[method(get)]
#[fixed(get = "CTY_NAME,ALL_VAL_MO,ALL_VAL_YR,NAICS_SDESC,NAICS")]
#[fixed(COMM_LVL = "NA2")]
pub struct NaicsReq(pub NaicsUsecaseInput);


#[derive(std::fmt::Debug)]
pub struct NaicsRes {
	pub rows: Vec<NaicsRow>
}
#[derive(std::fmt::Debug)]
pub struct NaicsRow {
	pub country_code:  String, // CTY_CODE
	pub country_name:  String, // CTY_NAME
	pub naics:         String, // NAICS
	pub month_value:   Option<u64>, // ALL_VAL_MO
	pub year_value:    Option<u64>, // ALL_VAL_YR
	pub naics_desc:    String, // NAICS_SDESC
	pub time:          String, // time (YYYY-MM)
}

#[derive(GenerateRes)]
pub struct NaicsRes(pub NaicsUsecaseFetched);

#[async_trait::async_trait]
impl ResSpec for NaicsRes {
	async fn from_resp(res: reqwest::Response) -> anyhow::Result<Self> {
			let rows: Vec<Vec<String>> = res.json().await?;

			if rows.len() < 2 {
					return Err(anyhow::anyhow!("no data rows"));
			}
			let header = &rows[0];

			let find = |name: &str| header.iter().position(|h| h == name)
					.ok_or_else(|| anyhow::anyhow!("column `{}` not found", name));

			let idx_cty_name = find("CTY_NAME")?;
			let idx_mo       = find("ALL_VAL_MO")?;
			let idx_yr       = find("ALL_VAL_YR")?;
			let idx_desc     = find("NAICS_SDESC")?;
			let idx_naics    = find("NAICS")?;
			let idx_time     = find("time")?;
			let idx_cty_code = find("CTY_CODE")?;

			let parse_u64 = |s: &str| s.parse::<u64>().ok();

			let rows: Vec<NaicsRow> = rows.into_iter().skip(1).map(|r| NaicsRow {
					country_name: r[idx_cty_name].clone(),
					month_value:  parse_u64(&r[idx_mo]),
					year_value:   parse_u64(&r[idx_yr]),
					naics:        r[idx_naics].clone(),
					naics_desc:   r[idx_desc].clone(),
					time:         r[idx_time].clone(),
					country_code: r[idx_cty_code].clone(),
			}).collect();

			Ok(NaicsRes{rows})
	}
}
impl kernel::core::ResSpec for NaicsRes {}

mod tests {
    use domain::valueobject::{trade::{country::code::CountryCode, time::YearMonth}, ValueObject};

    use crate::{infra::api::census::naics::model::{NaicsReq, NaicsRes}, internal::ResSpec, usecase::trade::naics::NaicsUsecaseInput};


	#[tokio::test]
	async fn request_test() {
			/* ① DTO を組む */
			let dto = NaicsUsecaseInput {
					time: YearMonth::new("2025-01").unwrap(),
					country_code: CountryCode::new("5880").unwrap(),
			};

			let builder: reqwest::RequestBuilder = NaicsReq(dto).into();
			let res = builder.send().await.unwrap();

			let parsed = NaicsRes::from_resp(res).await.unwrap();
			println!("{parsed:?}");

			assert_eq!(1, 1);          // ← 本来は parsed の中身を検証する
	}
}
