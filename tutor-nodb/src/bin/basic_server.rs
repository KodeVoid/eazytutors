// module imports
use actix_web::{web,App,HttpResponse,HttpServer,Responder};
use std::io;

// configure route
pub fn general_routes(cfg: &mut web::ServiceConfig){
	cfg.route("/health",web::get().to(health_check_handler));
}

// configure handler
pub async fn health_check_handler()->impl Responder {
	HttpResponse::Ok().json("Hello, Ezytutors is alive")
}

// Instantiaite the server
#[actix_rt::main]
async fn main()->io::Result<()>{
	// construct the app and configure routes
	let app = move || App::new().configure(general_routes);
	// Start HttpServer
	HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}