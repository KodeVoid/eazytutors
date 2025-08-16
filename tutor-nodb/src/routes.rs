use super::handlers::{
    get_course_details, get_tutor_courses_handler, health_check_handler, new_course_handler,
};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(new_course_handler))
            .route(
                "/tutor/{tutor_id}",
                web::get().to(get_tutor_courses_handler),
            )
            .route("/{course_id}", web::get().to(get_course_details)),
    );
}
