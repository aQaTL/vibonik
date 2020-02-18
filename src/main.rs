use actix_web::{App, HttpServer};
use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
	HttpServer::new(|| {
		App::new().service(actix_files::Files::new("/", "frontend").index_file("index.html"))
	})
	.bind("0.0.0.0:80")?
	.run()
	.await
}
