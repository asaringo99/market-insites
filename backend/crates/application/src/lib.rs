extern crate kernel;
pub mod infra;
pub mod usecase;

mod internal {
	pub(crate) trait ReqSpec: kernel::core::ReqSpec + Into<reqwest::RequestBuilder> {
		fn request(&self) -> impl serde::Serialize;
	}
	#[async_trait::async_trait]
	pub(crate) trait ResSpec: kernel::core::ResSpec {
		async fn from_resp(res: reqwest::Response) -> anyhow::Result<Self>;
	}
}