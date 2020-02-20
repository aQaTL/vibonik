use actix_web::{web, App, HttpServer, Scope};
use std::io;
use actix_cors::Cors;

#[actix_rt::main]
async fn main() -> io::Result<()> {
	dotenv::dotenv()
		.or_else(|_| {
			println!(".env not found, using .env_template");
			dotenv::from_filename(".env_template")
		})
		.expect("Failed to load .env");

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

	let mut server = HttpServer::new(|| {
		App::new()
			.service(
				Scope::new("/api")
					.wrap(Cors::new()
						.allowed_origin("http://localhost:8080")
						.finish()
					)
					.route("/echo", web::get().to(api::echo))
			)
			.service(actix_files::Files::new("/", "frontend/dist").index_file("index.html"))
	});

	for addr in bind_addresses {
		server = server
			.bind(&addr)
			.expect(&format!("failed to bind to {}", addr));
	}
	server.run().await
}

mod api {
	use actix_web::{Responder, web::Query};
	use serde::Deserialize;

	#[derive(Deserialize)]
	pub struct EchoParams {
		msg: Option<String>,
	}

	pub async fn echo(Query(params): Query<EchoParams>) -> impl Responder {
		params.msg.unwrap_or_default().to_string()
	}
}
