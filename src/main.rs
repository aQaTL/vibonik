use actix_cors::Cors;
use actix_files::NamedFile;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder, Scope};
use diesel::{
	r2d2::{self, ConnectionManager},
	PgConnection,
};
use failure::{Error, Fail};
use log::*;
use std::env;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod api;
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
			.wrap(IdentityService::new(
				CookieIdentityPolicy::new(&[0; 32])
					.name("auth-cookie")
					.domain(env::var("DOMAIN").unwrap())
					.secure(!cfg!(debug_assertions)),
			))
			.route("/echo", web::get().to(api::echo))
			.route("/auth", web::post().to(api::auth))
			.route("/get_user", web::get().to(api::get_user))
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
	let connspec = env::var("DATABASE_URL").expect("DATABASE_URL");
	let manager = ConnectionManager::<PgConnection>::new(connspec);
	r2d2::Pool::builder()
		.build(manager)
		.expect("Failed to create pool.")
}
