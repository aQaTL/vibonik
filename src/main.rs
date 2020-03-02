use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder, Scope};
use diesel::{
	r2d2::{self, ConnectionManager},
	PgConnection,
};
use failure::{Error, Fail};
use log::*;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod fb;
mod schema;

embed_migrations!();

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Fail, Debug)]
enum AppError {
	#[fail(display = "failed to load .env: {}", err)]
	DotEnv { err: dotenv::Error },
}

#[actix_rt::main]
async fn main() -> Result<(), Error> {
	dotenv::dotenv()
		.or_else(|_| {
			println!(".env not found, using .env_template");
			dotenv::from_filename(".env_template")
		})
		.map_err(|err| AppError::DotEnv { err })?;

	env_logger::init();

	info!("Connecting to the database");
	let pool = connect().await;

	embedded_migrations::run_with_output(&pool.get().unwrap(), &mut std::io::stdout())?;

	let mut addresses = std::env::vars()
		.filter(|(key, _)| key.starts_with("ADDRESS"))
		.map(|(_, val)| val)
		.collect::<Vec<String>>();
	if addresses.is_empty() {
		addresses.push("0.0.0.0".into());
	}

	let port = std::env::var("PORT").unwrap();

	let bind_addresses = addresses
		.into_iter()
		.map(|addr| format!("{}:{}", addr, port))
		.collect::<Vec<String>>();

	let mut server = HttpServer::new(move || {
		let app = App::new().wrap(Logger::new(r#" %a "%r" %s %T"#));

		let api = Scope::new("/api")
			.data(pool.clone())
			.route("/echo", web::get().to(api::echo))
			.route("/auth", web::post().to(api::auth))
			.route("/update_user", web::post().to(api::update_user));

		let app = if cfg!(debug_assertions) {
			app.service(
				api.wrap(
					Cors::new()
						.allowed_origin("http://localhost:8080")
						.allowed_origin("https://localhost:8080")
						.finish(),
				),
			)
		} else {
			app.service(api)
		};

		app.service(actix_files::Files::new("/", "frontend/dist").index_file("index.html"))
			.default_service(
				web::resource("").route(web::get().to(index)).route(
					web::route()
						.guard(guard::Not(guard::Get()))
						.to(HttpResponse::MethodNotAllowed),
				),
			)
	});

	for addr in bind_addresses {
		info!("Binding to {}", addr);
		server = server
			.bind(&addr)
			.expect(&format!("failed to bind to {}", addr));
	}
	Ok(server.run().await?)
}

async fn index() -> Result<impl Responder, actix_web::Error> {
	Ok(NamedFile::open("frontend/dist/index.html")?)
}

pub async fn connect() -> Pool {
	let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
	let manager = ConnectionManager::<PgConnection>::new(connspec);
	r2d2::Pool::builder()
		.build(manager)
		.expect("Failed to create pool.")
}

mod api {
	use crate::{db, fb, Pool};
	use actix_web::error::BlockingError;
	use actix_web::web::Data;
	use actix_web::{
		web,
		web::{Json, Query},
		HttpResponse, Responder,
	};
	use diesel::prelude::*;
	use diesel::result::Error;
	use log::error;
	use serde::{Deserialize, Serialize};
	use uuid::Uuid;

	#[derive(Deserialize)]
	pub struct EchoParams {
		msg: Option<String>,
	}

	pub async fn echo(Query(params): Query<EchoParams>) -> impl Responder {
		format!("Echo: {}", params.msg.unwrap_or_default().to_string())
	}

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	#[allow(dead_code)]
	pub struct AuthPayload {
		#[serde(rename = "userID")]
		user_id: String,
		access_token: String,
		expires_in: u64,
		signed_request: String,
	}

	#[derive(Serialize, Deserialize)]
	#[serde(rename_all = "camelCase", tag = "authStatus")]
	enum AuthStatus {
		Success(db::User),
		NewUser(db::User),
		Fail,
	}

	pub async fn auth(Json(data): Json<AuthPayload>, pool: Data<Pool>) -> impl Responder {
		let user = match fb::me(&data.access_token).await {
			Ok(fb::Response::User(user)) => user,
			Ok(fb::Response::Error { message, code, .. }) => {
				if code == 190 {
					return HttpResponse::Ok().json(AuthStatus::Fail);
				} else {
					error!("fb api error: {}", message);
					return HttpResponse::InternalServerError().body("");
				}
			}
			Err(e) => {
				error!("Failed to get user info from fb: {}", e);
				return HttpResponse::InternalServerError().body("");
			}
		};

		if user.id != data.user_id {
			return HttpResponse::Ok().json(AuthStatus::Fail);
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
			Ok(db_user) => ok_json(AuthStatus::Success(db_user)),
			Err(BlockingError::Error(Error::NotFound)) => {
				match web::block(move || {
					use crate::schema::users::dsl::*;
					let new_user = db::NewUser {
						fb_id: &data.user_id,
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
				.await
				{
					Ok(db_user) => ok_json(AuthStatus::NewUser(db_user)),
					Err(e) => {
						error!("failed to create user: {}", e);
						HttpResponse::InternalServerError().body("")
					}
				}
			}
			Err(e) => {
				//TODO: rely on Result::Error responder for returning errors?
				error!("failed to fetch user: {}", e);
				HttpResponse::InternalServerError().body("")
			}
		}
	}

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct UserData {
		#[serde(rename = "userID")]
		user_id: String,
		uuid: Uuid,
		//		access_token: String,
		//		name: String,
		food_preferences: Option<String>,
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
}
