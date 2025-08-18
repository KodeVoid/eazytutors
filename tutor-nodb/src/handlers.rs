use crate::models::{Course, Tutor};
use crate::state::AppState;
use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> impl Responder {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{health_check_response} {visit_count} times");
    *visit_count += 1;
    HttpResponse::Ok().body(response)
}

pub async fn create_new_tutor(
    app_state: web::Data<AppState>,
    tutor: web::Json<HashMap<String, String>>,
) -> impl Responder {
    let tutor_name = match tutor.get("name") {
        Some(name) => name.clone(),
        None => return HttpResponse::BadRequest().body("No tutor name provided"),
    };

    let tutor_email = match tutor.get("email") {
        Some(email) => email.clone(),
        None => return HttpResponse::BadRequest().body("No tutor email provided"),
    };

    let new_tutor = Tutor::new(tutor_name, tutor_email);
    let tutor_id = new_tutor.tutor_id;

    {
        let mut tutors = app_state.tutors.lock().unwrap();
        tutors.push(new_tutor);
    }

    HttpResponse::Ok().json(tutor_id) // return tutor_id as JSON
}

pub async fn get_tutor_id(
    app_state: web::Data<AppState>,
    tutor_details: web::Json<HashMap<String, String>>,
) -> impl Responder {
    let tutor_name = match tutor_details.get("name") {
        Some(name) => name,
        None => return HttpResponse::BadRequest().body("No name provided"),
    };

    let tutor_email = match tutor_details.get("email") {
        Some(email) => email,
        None => return HttpResponse::BadRequest().body("No email provided"),
    };

    let tutors = app_state.tutors.lock().unwrap();

    if let Some(tutor) = tutors
        .iter()
        .find(|t| t.name == *tutor_name && t.email == *tutor_email)
    {
        HttpResponse::Ok().json(tutor.tutor_id)
    } else {
        HttpResponse::NotFound().body(format!(
            "Tutor with name `{}` and email `{}` does not exist",
            tutor_name, tutor_email
        ))
    }
}

pub async fn new_course_handler(
    app_state: web::Data<AppState>,
    new_course: web::Json<HashMap<String, String>>,
) -> impl Responder {
    let tutor_id = match new_course.get("tutor_id") {
        Some(id_str) => match Uuid::parse_str(id_str) {
            Ok(id) => id,
            Err(_) => return HttpResponse::BadRequest().body("Invalid tutor_id format"),
        },
        None => return HttpResponse::BadRequest().body("No tutor_id provided"),
    };

    let course_name = match new_course.get("course_name") {
        Some(name) => name.clone(),
        None => return HttpResponse::BadRequest().body("No course_name provided"),
    };

    let course = Course::with_current_time(tutor_id, course_name);

    // Count existing courses for this tutor
    let course_count = {
        let courses = app_state.courses.lock().unwrap();
        courses.iter().filter(|c| c.tutor_id == tutor_id).count()
    };

    // Add new course
    {
        let mut courses = app_state.courses.lock().unwrap();
        courses.push(course);
    }

    HttpResponse::Ok().body(format!(
        "Added course for tutor {}, total courses: {}",
        tutor_id,
        course_count + 1
    ))
}

pub async fn get_tutor_courses_handler(
    app_state: web::Data<AppState>,
    params: web::Path<Uuid>,
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
    params: web::Path<Uuid>,
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
        None => HttpResponse::NotFound().body(format!("Course with ID {course_id} not found")),
    }
}
