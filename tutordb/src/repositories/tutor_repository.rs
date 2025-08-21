use crate::models::tutor::Tutor;
use sqlx::PgPool;
use uuid::Uuid;
use bigdecimal::BigDecimal;

pub async fn create_tutor(
    pool: &PgPool,
    name: String,
    email: String,
) -> Result<Tutor, sqlx::Error> {
    let tutor = Tutor::new(name, email);

    let inserted_tutor = sqlx::query_as!(
        Tutor,
        r#"
        INSERT INTO tutor (id, name, email, courses, rating)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, email, courses, rating as "rating:bigdecimal::BigDecimal"
        "#,
        tutor.id,
        tutor.name,
        tutor.email,
        tutor.courses,
        tutor.rating
    )
    .fetch_one(pool)
    .await?;

    Ok(inserted_tutor)
}

pub async fn find_tutor(
    tutor_id: Uuid,
    pool: &PgPool,
) -> Result<Tutor, sqlx::Error> {
    let tutor = sqlx::query_as!(
        Tutor,
        r#"
        SELECT id, name, email, courses, rating as "rating: bigdecimal::BigDecimal"
        FROM tutor
        WHERE id = $1
        "#,
        tutor_id
    )
    .fetch_one(pool)
    .await?;

    Ok(tutor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{PgPool, Executor};
    use uuid::Uuid;
    use bigdecimal::BigDecimal;

    // Helper function to create a temporary test database connection
    async fn setup_db() -> PgPool {
        // Make sure you set DATABASE_URL for a test DB
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for tests");

        let pool = PgPool::connect(&database_url).await.unwrap();

        // Optionally reset the table before each test
        pool.execute("TRUNCATE TABLE tutor RESTART IDENTITY;")
            .await
            .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_create_tutor() {
        let pool = setup_db().await;

        let name = "Alice".to_string();
        let email = "alice@example.com".to_string();

        let tutor = create_tutor(&pool, name.clone(), email.clone())
            .await
            .expect("Failed to create tutor");

        assert_eq!(tutor.name, name);
        assert_eq!(tutor.email, email);
        assert_eq!(tutor.courses, 0);
        assert_eq!(tutor.rating, BigDecimal::from(0));
        assert!(!tutor.id.is_nil());
    }

    #[tokio::test]
    async fn test_find_tutor() {
        let pool = setup_db().await;

        // First insert a tutor
        let name = "Bob".to_string();
        let email = "bob@example.com".to_string();

        let created_tutor = create_tutor(&pool, name.clone(), email.clone())
            .await
            .expect("Failed to create tutor");

        // Now fetch it by id
        let fetched_tutor = find_tutor(created_tutor.id, &pool)
            .await
            .expect("Failed to find tutor");

        assert_eq!(fetched_tutor.id, created_tutor.id);
        assert_eq!(fetched_tutor.name, name);
        assert_eq!(fetched_tutor.email, email);
        assert_eq!(fetched_tutor.courses, 0);
        assert_eq!(fetched_tutor.rating, BigDecimal::from(0));
    }

    #[tokio::test]
    async fn test_find_tutor_not_found() {
        let pool = setup_db().await;

        let random_id = Uuid::new_v4();
        let result = find_tutor(random_id, &pool).await;

        assert!(result.is_err(), "Expected error for non-existent tutor");
    }
}
