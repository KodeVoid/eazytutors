use std::net::TcpListener;
use tutor_nodb::{run, connect_db};
use serde_json;
use uuid::Uuid;

async fn spawn_app() -> String {
    // Bind to a random available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let pool = connect_db().await.expect("Failed to connect to DB");

    let server = run(listener, pool).expect("Failed to start server");
    // Spawn the server on a background task
    tokio::spawn(server);
    
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Spawn app
    let address = spawn_app().await;
    
    // Give the server a moment to start up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert!(response.status().is_success());
    assert_eq!(200, response.status().as_u16());
    
    // Check response body contains expected message
    let body = response.text().await.expect("Failed to read response body");
    assert!(body.contains("Tutor Services running fine"));
}

#[tokio::test]
async fn test_course_creation() {
    let address = spawn_app().await;
    
    // Give the server a moment to start up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let client = reqwest::Client::new();
    
    // First create a tutor
    let tutor = serde_json::json!({
        "name": "kendrick",
        "email": "kendrick@gmail.com"
    });
    
    let tutor_response = client
        .post(&format!("{}/tutors/", &address))
        .header("Content-Type", "application/json")
        .json(&tutor)
        .send()
        .await
        .expect("Failed to create tutor");
    
    assert!(tutor_response.status().is_success());
    
    // Get the tutor_id from the response
    let tutor_id: Uuid = tutor_response
        .json()
        .await
        .expect("Failed to parse tutor_id");
    
    // Test POST to create a course with correct structure
    let new_course = serde_json::json!({
        "tutor_id": tutor_id.to_string(),
        "course_name": "Test Course for Integration Testing"
    });
    
    let response = client
        .post(&format!("{}/courses/", &address))
        .header("Content-Type", "application/json")
        .json(&new_course)
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert!(response.status().is_success());
    assert_eq!(200, response.status().as_u16());
    
    // Check response body
    let body = response.text().await.expect("Failed to read response body");
    assert!(body.contains(&format!("Added course for tutor {}", tutor_id)));
}

#[tokio::test]
async fn test_get_tutor_courses() {
    let address = spawn_app().await;
    
    // Give the server a moment to start up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let client = reqwest::Client::new();
    
    // First create a tutor
    let tutor = serde_json::json!({
        "name": "test_tutor",
        "email": "test@example.com"
    });
    
    let tutor_response = client
        .post(&format!("{}/tutors/", &address))
        .header("Content-Type", "application/json")
        .json(&tutor)
        .send()
        .await
        .expect("Failed to create tutor");
    
    let tutor_id: Uuid = tutor_response
        .json()
        .await
        .expect("Failed to parse tutor_id");
    
    // Create a course for this tutor
    let new_course = serde_json::json!({
        "tutor_id": tutor_id.to_string(),
        "course_name": "Test Course"
    });
    
    let _create_response = client
        .post(&format!("{}/courses/", &address))
        .header("Content-Type", "application/json")
        .json(&new_course)
        .send()
        .await
        .expect("Failed to create course.");
    
    // Then, retrieve courses for this tutor using the correct endpoint
    let response = client
        .get(&format!("{}/tutors/{}/courses", &address, tutor_id))
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert!(response.status().is_success());
    assert_eq!(200, response.status().as_u16());
    
    // Parse JSON response
    let courses: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");
    
    // Should be an array with one course
    assert!(courses.is_array());
    let courses_array = courses.as_array().unwrap();
    assert_eq!(1, courses_array.len());
    
    // Check course details
    let course = &courses_array[0];
    assert_eq!(tutor_id.to_string(), course["tutor_id"].as_str().unwrap());
    assert_eq!("Test Course", course["course_name"]);
    assert!(course["posted_time"].is_string());
    assert!(course["course_id"].is_string());
}

#[tokio::test]
async fn test_get_course_details() {
    let address = spawn_app().await;
    
    // Give the server a moment to start up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let client = reqwest::Client::new();
    
    // First create a tutor
    let tutor = serde_json::json!({
        "name": "detail_tutor",
        "email": "detail@example.com"
    });
    
    let tutor_response = client
        .post(&format!("{}/tutors/", &address))
        .header("Content-Type", "application/json")
        .json(&tutor)
        .send()
        .await
        .expect("Failed to create tutor");
    
    let tutor_id: Uuid = tutor_response
        .json()
        .await
        .expect("Failed to parse tutor_id");
    
    // Create a course
    let new_course = serde_json::json!({
        "tutor_id": tutor_id.to_string(),
        "course_name": "Detailed Test Course"
    });
    
    let create_response = client
        .post(&format!("{}/courses/", &address))
        .header("Content-Type", "application/json")
        .json(&new_course)
        .send()
        .await
        .expect("Failed to create course.");
    
    assert!(create_response.status().is_success());
    
    // Get the courses to find the course_id
    let courses_response = client
        .get(&format!("{}/tutors/{}/courses", &address, tutor_id))
        .send()
        .await
        .expect("Failed to get courses");
    
    let courses: serde_json::Value = courses_response
        .json()
        .await
        .expect("Failed to parse courses");
    
    let course_id = courses[0]["course_id"].as_str().unwrap();
    
    // Then, get specific course details
    let response = client
        .get(&format!("{}/courses/{}", &address, course_id))
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert!(response.status().is_success());
    
    // Parse JSON response
    let course: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");
    
    assert_eq!(tutor_id.to_string(), course["tutor_id"].as_str().unwrap());
    assert_eq!(course_id, course["course_id"].as_str().unwrap());
    assert_eq!("Detailed Test Course", course["course_name"]);
    assert!(course["posted_time"].is_string());
}

#[tokio::test]
async fn test_course_not_found() {
    let address = spawn_app().await;
    
    // Give the server a moment to start up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let client = reqwest::Client::new();
    
    // Generate a random UUID that definitely doesn't exist
    let non_existent_id = Uuid::new_v4();
    
    // Try to get a non-existent course
    let response = client
        .get(&format!("{}/courses/{}", &address, non_existent_id))
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert_eq!(404, response.status().as_u16());
    
    let body = response.text().await.expect("Failed to read response body");
    assert!(body.contains(&format!("Course with ID {} not found", non_existent_id)));
}

#[tokio::test]
async fn test_get_tutor_id_by_details() {
    let address = spawn_app().await;
    
    // Give the server a moment to start up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let client = reqwest::Client::new();
    
    // First create a tutor
    let tutor = serde_json::json!({
        "name": "lookup_tutor",
        "email": "lookup@example.com"
    });
    
    let tutor_response = client
        .post(&format!("{}/tutors/", &address))
        .header("Content-Type", "application/json")
        .json(&tutor)
        .send()
        .await
        .expect("Failed to create tutor");
    
    let expected_tutor_id: Uuid = tutor_response
        .json()
        .await
        .expect("Failed to parse tutor_id");
    
    // Now lookup the tutor by name and email
    let lookup_data = serde_json::json!({
        "name": "lookup_tutor",
        "email": "lookup@example.com"
    });
    
    let lookup_response = client
        .post(&format!("{}/tutors/id", &address))
        .header("Content-Type", "application/json")
        .json(&lookup_data)
        .send()
        .await
        .expect("Failed to lookup tutor");
    
    assert!(lookup_response.status().is_success());
    
    let found_tutor_id: Uuid = lookup_response
        .json()
        .await
        .expect("Failed to parse found tutor_id");
    
    assert_eq!(expected_tutor_id, found_tutor_id);
}