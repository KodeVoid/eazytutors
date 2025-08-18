use sqlx::Postgres;
use sqlx::Pool;
use sqlx::postgres::{PgPool,PgPoolOptions};
use actix_web::{App, HttpServer, dev::Server, web};
use std::io;
use std::net::TcpListener;
use dotenvy::dotenv;
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

pub fn run(listener: TcpListener,db_pool:PgPool) -> Result<Server, io::Error> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "Tutor Services running fine".to_string(),
        visit_count: Mutex::new(0u32),
        courses: Mutex::new(vec![]),
        db_pool:db_pool
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

pub async fn connect_db()->Result<Pool<Postgres>,sqlx::Error> {
    dotenv().ok();
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(value)=>{value},
        Err(e)=>{eprintln!("Failed to load DATABASE_URL from environment {e}");
        return Err(sqlx::Error::Configuration(Box::new(e)))
    }
    };

    let pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await?;
    Ok(pool)
}