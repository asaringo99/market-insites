use async_trait;
use anyhow;

#[async_trait::async_trait]
pub trait ReqSpec: Send + Sync {}

#[async_trait::async_trait]
pub trait ResSpec: Sized + Send + Sync + 'static {}

#[async_trait::async_trait]
pub trait Pull<Req: ReqSpec, Res: ResSpec> {
		async fn pull(&self, req: Req) -> anyhow::Result<Res>;
}

#[async_trait::async_trait]
pub trait Push<S: ResSpec, T: From<S>> {
	async fn push(&self, items: &[T]) -> anyhow::Result<()>;
}
