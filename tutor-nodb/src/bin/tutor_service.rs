use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::io;
use std::sync::Mutex;
#[path = "../handlers.rs"]
mod handlers;
#[path = "../state.rs"]
mod state;

#[path = "../models.rs"]
mod models;
use handlers::general_routes;
use models::Course;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "Tutor Services running fine".to_string(),
        visit_count: Mutex::new(0u32),
        courses: Mutex::new(vec![]),
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
