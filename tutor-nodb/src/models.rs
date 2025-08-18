use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Course {
    pub tutor_id: Uuid,
    pub course_id: Uuid,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        match course.posted_time {
            Some(time) => Self::new(
                course.tutor_id,
                
                course.course_name.clone(),
                Some(time),
            ),
            None => Self::with_current_time(
                course.tutor_id,
                course.course_name.clone(),
            ),
        }
    }
}
impl Course {
    pub fn new(
        tutor_id: Uuid,
        course_name: String,
        posted_time: Option<NaiveDateTime>,
    ) -> Self {
        Course {
            tutor_id,
            course_id:Uuid::new_v4(),
            course_name,
            posted_time,
        }
    }

    pub fn with_current_time(tutor_id: Uuid, course_name: String) -> Self {
        Course {
            tutor_id,
            course_id:Uuid::new_v4(),
            course_name,
            posted_time: Some(chrono::Utc::now().naive_utc()),
        }
    }

    pub fn update_posted_time(&mut self) {
        self.posted_time = Some(chrono::Utc::now().naive_utc());
    }

    pub fn is_posted_by_tutor(&self, tutor_id: Uuid) -> bool {
        self.tutor_id == tutor_id
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Tutor {
    pub name:String,
    pub email:String,
    pub tutor_id:Uuid,
}

impl Tutor{
    pub fn new(name:String,email:String) ->Self{
        Tutor{
            name,
            email,
            tutor_id:Uuid::new_v4()}
}}
