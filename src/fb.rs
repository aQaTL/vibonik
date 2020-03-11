#![allow(dead_code)]

use failure::Fail;
use log::*;
use serde::Deserialize;
use url::Url;

macro_rules! API {
	() => {
		"https://graph.facebook.com/v6.0"
	};
}
const API: &str = API!();

#[derive(Deserialize)]
pub enum Response {
	#[serde(rename = "error")]
	Error(Error),
	User(User),
}

#[derive(Deserialize, Debug)]
pub struct Error {
	pub message: String,
	#[serde(rename = "type")]
	pub err_type: String,
	pub code: u32,
	pub error_subcode: u32,
	pub fbtrace_id: String,
}

#[derive(Deserialize)]
pub struct User {
	pub name: String,
	pub id: String,
}

#[derive(Fail, Debug)]
enum FbError {
	#[fail(display = "general error")]
	General,
}

pub async fn me(access_token: &str) -> Result<Response, failure::Error> {
	let url = Url::parse_with_params(concat!(API!(), "/me"), &[("access_token", &access_token)]);
	let url = match url {
		Ok(url) => url,
		Err(e) => {
			error!("failed to create url to /me: {}", e);
			return Err(FbError::General.into());
		}
	};
	let response = reqwest::get(url).await?;
	let bytes = response.bytes().await?;
	match serde_json::from_slice::<User>(&bytes) {
		Ok(user) => Ok(Response::User(user)),
		Err(_) => Ok(serde_json::from_slice::<Response>(&bytes)?),
	}
}
