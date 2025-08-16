use super::state::AppState;
use actix_web::{HttpResponse, Responder, web};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

async fn health_check_handler(app_state: web::Data<AppState>) -> impl Responder {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    drop(visit_count);
    HttpResponse::Ok().json(response)
}
