extern crate kernel;

use kernel::core::{Pull, Push};
use async_trait;
use anyhow;

use crate::{internal::ResSpec, usecase::trade::naics::NaicsUsecaseInput};

use super::model::{NaicsReq, NaicsRes};

pub struct NaicsFetcher;

#[async_trait::async_trait]
impl Pull<NaicsUsecaseInput, NaicsRes> for NaicsFetcher {
	async fn pull(&self, req: NaicsUsecaseInput) -> anyhow::Result<NaicsRes> {
		let req: reqwest::RequestBuilder = NaicsReq(req).into();
		let res = req.send().await?;
		let res = NaicsRes::from_resp(res).await?;
		anyhow::Ok(res)
	}
}