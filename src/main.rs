use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Scope};
use diesel::{
	r2d2::{self, ConnectionManager},
	PgConnection,
};
use std::io;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
	dotenv::dotenv()
		.or_else(|_| {
			println!(".env not found, using .env_template");
			dotenv::from_filename(".env_template")
		})
		.expect("Failed to load .env");

	println!("Connecting to the database");
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
			.service(
				Scope::new("/api")
					.data(pool.clone())
					.wrap(Cors::new().allowed_origin("http://localhost:8080").finish())
					.route("/echo", web::get().to(api::echo)),
			)
			.service(actix_files::Files::new("/", "frontend/dist").index_file("index.html"))
	});

	for addr in bind_addresses {
		println!("Binding to {}", addr);
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
	use actix_web::{web::Query, Responder};
	use serde::Deserialize;

	#[derive(Deserialize)]
	pub struct EchoParams {
		msg: Option<String>,
	}

	pub async fn echo(Query(params): Query<EchoParams>) -> impl Responder {
		format!("Echo: {}", params.msg.unwrap_or_default().to_string())
	}
}
