use crate::model::ResCodeBody;
use anyhow::{Context, Result};
use hyper::{Body, Method, Request, Uri};
use std::collections::HashMap;
pub async fn test() -> Result<()> {
	let req = Request::builder()
		.method(Method::GET)
		.uri("http://api.waizaowang.com/doc/getStockHSABaseInfo?code=000001&fields=code,name,market,bk,roe,zgb,ltgb,ltsz,zsz,ssdate,hsgt&export=3&token=febb869f0979d084c4a8d17ce45ea866")
		.header("content-type", "application/json") //.body(None)?;
		.body(Body::from("hello"))?;
	let client = hyper::Client::new();
	let resp = client.request(req).await?;
	let a = hyper::body::to_bytes(resp.into_body()).await?;
	println!("{:?}", a);
	let c = serde_json::from_slice::<ResCodeBody>(&a)?;
	println!("{:?}", c);
	Ok(())
}
