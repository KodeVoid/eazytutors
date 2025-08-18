use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(new_course_handler)) // POST /courses
            .route("/{course_id}", web::get().to(get_course_details)), // GET /courses/{id}
    );

    cfg.service(
        web::scope("/tutors")
            .route("/", web::post().to(create_new_tutor)) // POST /tutors
            .route("/id", web::post().to(get_tutor_id)) // POST /tutors/id (lookup by name/email)
            .route("/{tutor_id}/courses", web::get().to(get_tutor_courses_handler)), // GET /tutors/{id}/courses
    );
}
