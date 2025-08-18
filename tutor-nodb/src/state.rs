use sqlx::Postgres;
use std::sync::Mutex;
use sqlx::Pool;
use super::models::Course;
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
   pub db_pool:Pool<Postgres>,
}
