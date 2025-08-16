use super::state::AppState;
use crate::models::Course;
use actix_web::{HttpRequest, HttpResponse, Responder, web};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> impl Responder {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{health_check_response} {visit_count} times" );
    *visit_count += 1;
    HttpResponse::Ok().json(response)
}

pub async fn new_course_handler(
    app_state: web::Data<AppState>,
    new_course: web::Json<Course>,
) -> impl Responder {
    println!("Received new Course");

    let course: Course = new_course.into();
    let tutor_id = course.tutor_id;

    // Count existing courses for this tutor
    let course_count = {
        let courses = app_state.courses.lock().unwrap();
        courses.iter().filter(|c| c.tutor_id == tutor_id).count()
    };

    // Add the new course
    app_state.courses.lock().unwrap().push(course);

    HttpResponse::Ok().json(format!(
        "Added course for tutor {}, total courses for this tutor: {}",
        tutor_id,
        course_count + 1
    ))
}

pub async fn get_tutor_courses_handler(
    app_state: web::Data<AppState>,
    params: web::Path<u32>,
) -> impl Responder {
    let tutor_id = params.into_inner();

    let courses: Vec<Course> = {
        let courses = app_state.courses.lock().unwrap();
        courses
            .iter()
            .filter(|course| course.tutor_id == tutor_id)
            .cloned()
            .collect()
    };

    HttpResponse::Ok().json(courses)
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<u32>,
) -> impl Responder {
    let course_id = params.into_inner();

    let course: Option<Course> = {
        let courses = app_state.courses.lock().unwrap();
        courses
            .iter()
            .find(|course| course.course_id == course_id)
            .cloned()
    };

    match course {
        Some(c) => HttpResponse::Ok().json(c),
        None => HttpResponse::NotFound().json(format!("Course with ID {course_id} not found", )),
    }
}
