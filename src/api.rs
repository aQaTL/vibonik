use crate::{db, fb, Pool};
use actix_http::http::StatusCode;
use actix_http::ResponseError;
use actix_identity::Identity;
use actix_web::dev::{HttpResponseBuilder, Payload};
use actix_web::error::BlockingError;
use actix_web::web::Data;
use actix_web::{
	web,
	web::{Json, Query},
	FromRequest, HttpRequest, HttpResponse, Responder,
};
use diesel::prelude::*;
use failure::Fail;
use futures::future;
use log::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Fail, Debug)]
pub enum ApiError {
	#[fail(display = "internal server error: {}", _0)]
	InternalServerError(failure::Error),
	#[fail(display = "FbError: {:?}", _0)]
	FbError(fb::Error),
	#[fail(display = "database error")]
	DbError(diesel::result::Error),
}

impl ResponseError for ApiError {
	fn status_code(&self) -> StatusCode {
		match self {
			ApiError::DbError(diesel::result::Error::NotFound) => StatusCode::NOT_FOUND,
			_ => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}

	fn error_response(&self) -> HttpResponse {
		error!("API Error: {:?}", self);
		match self {
			_ => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
				.json(failure::Fail::name(self)),
		}
	}
}

impl From<diesel::result::Error> for ApiError {
	fn from(err: diesel::result::Error) -> Self {
		ApiError::DbError(err)
	}
}

impl<T> From<BlockingError<T>> for ApiError
where
	T: 'static
		+ std::fmt::Debug
		+ std::fmt::Display
		+ std::marker::Send
		+ std::marker::Sync
		+ Into<ApiError>,
{
	fn from(err: BlockingError<T>) -> Self {
		match err {
			BlockingError::Error(e) => e.into(),
			BlockingError::Canceled => ApiError::InternalServerError(failure::err_msg(err)),
		}
	}
}

impl From<AuthStatus> for HttpResponse {
	fn from(status: AuthStatus) -> HttpResponse {
		HttpResponse::Ok().json(status)
	}
}

#[derive(Deserialize)]
pub struct EchoParams {
	msg: Option<String>,
}

pub async fn echo(Query(params): Query<EchoParams>) -> impl Responder {
	format!("Echo: {}", params.msg.unwrap_or_default().to_string())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AuthPayload {
	FacebookAuth(FacebookAuth),
	FormAuth(FormAuth),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct FacebookAuth {
	#[serde(rename = "userID")]
	user_id: String,
	access_token: String,
	expires_in: u64,
	signed_request: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormAuth {
	login: String,
	password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "authStatus")]
enum AuthStatus {
	Success(db::User),
	NewUser(db::User),
	Fail,
}

pub async fn auth(
	Json(data): Json<AuthPayload>,
	id: Identity,
	pool: Data<Pool>,
) -> Result<HttpResponse, ApiError> {
	match data {
		AuthPayload::FacebookAuth(data) => facebook_auth(data, id, pool.get_ref()).await,
		AuthPayload::FormAuth(data) => form_auth(data, id, pool.get_ref()).await,
	}
}

async fn facebook_auth(
	data: FacebookAuth,
	id: Identity,
	pool: &Pool,
) -> Result<HttpResponse, ApiError> {
	let user = match fb::me(&data.access_token).await {
		Ok(fb::Response::User(user)) => user,
		Ok(fb::Response::Error(fb::Error { code, .. })) if code == 190 => {
			return Ok(AuthStatus::Fail.into());
		}
		Ok(fb::Response::Error(err)) => {
			return Err(ApiError::FbError(err));
		}
		Err(e) => {
			return Err(ApiError::InternalServerError(e));
		}
	};

	if user.id != data.user_id {
		return Ok(AuthStatus::Fail.into());
	}

	let data = std::sync::Arc::new(data);
	let db_user = {
		let pool = pool.clone();
		let data = data.clone();
		web::block(move || {
			use crate::schema::users::dsl::*;
			users
				.filter(fb_id.eq(&data.user_id))
				.get_result::<db::User>(&pool.get().unwrap())
		})
		.await
	};

	let pool = pool.clone();
	match db_user {
		Ok(db_user) => Ok(AuthStatus::Success(db_user).into()),
		Err(BlockingError::Error(diesel::result::Error::NotFound)) => {
			let db_user = web::block(move || {
				use crate::schema::users::dsl::*;
				let new_user = db::NewUser {
					fb_id: Some(&data.user_id),
					access_token: Some(&data.access_token),
					name: &user.name,
					uuid: Uuid::new_v4(),
					role: Some(db::Role::User),
					..Default::default()
				};
				diesel::insert_into(users)
					.values(&new_user)
					.get_result::<db::User>(&pool.get().unwrap())
			})
			.await?;
			Ok(AuthStatus::NewUser(db_user).into())
		}
		Err(e) => Err(ApiError::InternalServerError(failure::err_msg(e))),
	}
}

async fn form_auth(data: FormAuth, id: Identity, pool: &Pool) -> Result<HttpResponse, ApiError> {
	let pool = pool.clone();

	let user: db::User = web::block(move || {
		use crate::schema::users::dsl::*;

		users
			.filter(login.eq(&data.login))
			.filter(password_hash.eq(crate::hash_password(&data.password)))
			.get_result::<db::User>(&pool.get().unwrap())
	})
	.await?;

	id.remember(user.uuid.to_string());
	Ok(AuthStatus::NewUser(user).into())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
	#[serde(rename = "userID")]
	user_id: String,
	uuid: Uuid,
	//		access_token: String,
	//		name: String,
	food_preferences: Option<String>,
}

pub struct LoggedUser;

impl FromRequest for LoggedUser {
	type Error = actix_web::Error;
	type Future = future::Ready<Result<LoggedUser, Self::Error>>;
	type Config = ();

	fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
		if let Ok(id) = Identity::from_request(req, pl).into_inner() {
			if let Some(_) = id.identity() {
				return future::ok(LoggedUser);
			}
		}
		future::err(HttpResponse::Unauthorized().body("").into())
	}
}

pub async fn get_user(_: LoggedUser) -> HttpResponse {
	ok_json(UserData {
		user_id: "adf".to_string(),
		uuid: Uuid::new_v4(),
		food_preferences: None,
	})
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "updateStatus")]
enum UpdateStatus {
	Success,
	Fail,
}

pub async fn update_user(Json(form): Json<UserData>, pool: Data<Pool>) -> impl Responder {
	let insert = web::block(move || {
		let conn = pool.get().unwrap();

		use crate::schema::users::dsl::*;
		diesel::update(users.filter(uuid.eq(form.uuid)))
			.set((food_preferences.eq(form.food_preferences)))
			.execute(&conn)
	})
	.await;

	match insert {
		Ok(_rows_affected) => (),
		Err(err) => {
			error!("failed to insert user: {}", err);
			return ok_json(UpdateStatus::Fail);
		}
	}

	ok_json(UpdateStatus::Success)
}

fn ok_json<T: Serialize>(value: T) -> actix_http::Response {
	HttpResponse::Ok().json(value)
}
