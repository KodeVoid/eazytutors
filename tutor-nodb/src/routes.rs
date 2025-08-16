use actix_web::{web};
use super::handlers::{
    health_check_handler,
    new_course_handler,
    get_tutor_courses_handler,
    get_course_details
};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(new_course_handler))
            .route("/tutor/{tutor_id}", web::get().to(get_tutor_courses_handler))
            .route("/{course_id}", web::get().to(get_course_details))
    );
}