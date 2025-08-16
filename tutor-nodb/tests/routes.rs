use std::net::TcpListener;
use tutor_nodb::run;
use serde_json;

fn spawn_app() -> String {
    // Bind to a random available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to start server");
    
    // Spawn the server on a background task
    tokio::spawn(server);
    
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Spawn app
    let address = spawn_app();
    
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
    let address = spawn_app();
    
    // Give the server a moment to start up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let client = reqwest::Client::new();
    
    // Test POST to create a course with correct structure
    let new_course = serde_json::json!({
        "tutor_id": 123,
        "course_id": 456,
        "course_name": "Test Course for Integration Testing"
    });
    
    let response = client
        .post(&format!("{}/courses/", address))
        .header("Content-Type", "application/json")
        .json(&new_course)
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert!(response.status().is_success());
    assert_eq!(200, response.status().as_u16());
    
    // Check response body
    let body = response.text().await.expect("Failed to read response body");
    assert!(body.contains("Added course for tutor 123"));
}

#[tokio::test]
async fn test_get_tutor_courses() {
    let address = spawn_app();
    
    // Give the server a moment to start up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let client = reqwest::Client::new();
    
    // First, create a course
    let new_course = serde_json::json!({
        "tutor_id": 123,
        "course_id": 456,
        "course_name": "Test Course"
    });
    
    let _create_response = client
        .post(&format!("{}/courses/", address))
        .header("Content-Type", "application/json")
        .json(&new_course)
        .send()
        .await
        .expect("Failed to create course.");
    
    // Then, retrieve courses for this tutor
    let response = client
        .get(&format!("{}/courses/tutor/123", address))
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
    assert_eq!(123, course["tutor_id"]);
    assert_eq!(456, course["course_id"]);
    assert_eq!("Test Course", course["course_name"]);
    assert!(course["posted_time"].is_string());
}

#[tokio::test]
async fn test_get_course_details() {
    let address = spawn_app();
    
    // Give the server a moment to start up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let client = reqwest::Client::new();
    
    // First, create a course
    let new_course = serde_json::json!({
        "tutor_id": 123,
        "course_id": 789,
        "course_name": "Detailed Test Course"
    });
    
    let _create_response = client
        .post(&format!("{}/courses/", address))
        .header("Content-Type", "application/json")
        .json(&new_course)
        .send()
        .await
        .expect("Failed to create course.");
    
    // Then, get specific course details
    let response = client
        .get(&format!("{}/courses/789", address))
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert!(response.status().is_success());
    
    // Parse JSON response
    let course: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");
    
    assert_eq!(123, course["tutor_id"]);
    assert_eq!(789, course["course_id"]);
    assert_eq!("Detailed Test Course", course["course_name"]);
    assert!(course["posted_time"].is_string());
}

#[tokio::test]
async fn test_course_not_found() {
    let address = spawn_app();
    
    // Give the server a moment to start up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let client = reqwest::Client::new();
    
    // Try to get a non-existent course
    let response = client
        .get(&format!("{}/courses/999", address))
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert_eq!(404, response.status().as_u16());
    
    let body = response.text().await.expect("Failed to read response body");
    assert!(body.contains("Course with ID 999 not found"));
}