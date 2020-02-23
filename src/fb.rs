#![allow(dead_code)]
use failure::{Error, Fail};
use log::*;
use serde::Deserialize;
use url::Url;

macro_rules! API {
	() => {
		"https://graph.facebook.com/v6.0"
	};
}
const API: &'static str = API!();

#[derive(Deserialize)]
pub enum Response {
	#[serde(rename = "error")]
	Error {
		message: String,
		#[serde(rename = "type")]
		err_type: String,
		code: u32,
		error_subcode: u32,
		fbtrace_id: String,
	},
	User(User),
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

pub async fn me(access_token: String) -> Result<Response, Error> {
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
