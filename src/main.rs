use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Scope};
use diesel::{
	r2d2::{self, ConnectionManager},
	PgConnection,
};
use log::*;
use std::io;

mod fb;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
	dotenv::dotenv()
		.or_else(|_| {
			println!(".env not found, using .env_template");
			dotenv::from_filename(".env_template")
		})
		.expect("Failed to load .env");

	env_logger::init();

	info!("Connecting to the database");
	let pool = connect().await;

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
		App::new()
			.wrap(Logger::new(r#" %a "%r" %s %T"#))
			.service(
				Scope::new("/api")
					.data(pool.clone())
					.wrap(
						Cors::new()
							.allowed_origin("http://localhost:8080")
							.allowed_origin("https://localhost:8080")
							.finish(),
					)
					.route("/echo", web::get().to(api::echo))
					.route("/auth", web::post().to(api::auth))
					.route("/signup", web::post().to(api::signup)),
			)
			.service(actix_files::Files::new("/", "frontend/dist").index_file("index.html"))
	});

	for addr in bind_addresses {
		info!("Binding to {}", addr);
		server = server
			.bind(&addr)
			.expect(&format!("failed to bind to {}", addr));
	}
	server.run().await
}

pub async fn connect() -> Pool {
	let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
	let manager = ConnectionManager::<PgConnection>::new(connspec);
	r2d2::Pool::builder()
		.build(manager)
		.expect("Failed to create pool.")
}

mod api {
	use crate::fb;
	use actix_web::{
		web::{Json, Query},
		HttpResponse, Responder,
	};
	use log::error;
	use serde::{Deserialize, Serialize};

	#[derive(Deserialize)]
	pub struct EchoParams {
		msg: Option<String>,
	}

	pub async fn echo(Query(params): Query<EchoParams>) -> impl Responder {
		format!("Echo: {}", params.msg.unwrap_or_default().to_string())
	}

	#[derive(Deserialize)]
	#[allow(dead_code)]
	pub struct AuthPayload {
		#[serde(rename = "userID")]
		user_id: String,
		#[serde(rename = "accessToken")]
		access_token: String,
		#[serde(rename = "expiresIn")]
		expires_in: u64,
		#[serde(rename = "signedRequest")]
		signed_request: String,
	}

	#[derive(Serialize, Deserialize)]
	enum AuthStatus {
		Success,
		UserNotFound,
		Fail,
	}

	pub async fn auth(Json(data): Json<AuthPayload>) -> impl Responder {
		let user = match fb::me(data.access_token).await {
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
		HttpResponse::Ok().json(AuthStatus::Success)
	}

	pub async fn signup() -> impl Responder {
		""
	}
}
