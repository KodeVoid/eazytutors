use std::net::TcpListener;
use actix_web::{App, HttpServer, web, dev::Server};
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../state.rs"]
mod state;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;

use routes::{general_routes, course_routes};
use state::AppState;

fn run(listener: TcpListener) -> Result<Server, io::Error> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "Tutor Services running fine".to_string(),
        visit_count: Mutex::new(0u32),
        courses: Mutex::new(vec![]),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    let server = HttpServer::new(app).listen(listener)?.run();
    Ok(server)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    run(listener)?.await
}

