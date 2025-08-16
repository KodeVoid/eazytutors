use actix_web::{App, HttpServer, dev::Server, web};
use std::io;
use std::net::TcpListener;
use std::sync::Mutex;

#[path = "handlers.rs"]
mod handlers;
#[path = "models.rs"]
mod models;
#[path = "routes.rs"]
mod routes;
#[path = "state.rs"]
mod state;

use routes::{course_routes, general_routes};
use state::AppState;

pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
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
