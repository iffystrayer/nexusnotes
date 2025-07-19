#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use crate::models::Note;
    use sqlx::{SqlitePool, migrate::Migrator};
    use std::path::Path;

    // Helper to create an in-memory database for testing
    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.expect("Failed to connect to in-memory DB");
        // Run migrations
        let migrator = Migrator::new(Path::new("./migrations")).await.expect("Failed to create migrator");
        migrator.run(&pool).await.expect("Failed to run migrations");
        pool
    }

    #[tokio::test]
    async fn test_create_note() {
        let pool = setup_test_db().await;
        // Ensure the pool is used by the init_db function for tests
        // This might require modifying init_db to accept a pool or use a global/thread-local pool
        // For now, we'll assume init_db connects to the same in-memory DB if called without path
        // Or, we can directly use the pool for the test
        
        // Create a dummy notebook for the note
        let notebook_id = new_id();
        sqlx::query("INSERT INTO notebooks (id, title) VALUES ($1, $2)")
            .bind(&notebook_id)
            .bind("Test Notebook")
            .execute(&pool)
            .await
            .expect("Failed to create test notebook");

        let payload = NotePayload {
            title: "Test Note".to_string(),
            notebook_id: notebook_id.clone(),
            markdown: "This is a test note.".to_string(),
        };

        let result = create_note(payload).await;
        assert!(result.is_ok());

        let created_note = result.unwrap();
        assert_eq!(created_note.title, "Test Note");
        assert_eq!(created_note.notebook_id, notebook_id);
        assert_eq!(created_note.markdown, "This is a test note.");

        // Verify the note exists in the database
        let fetched_note: Note = sqlx::query_as::<_, Note>(
            "SELECT id, notebook_id, title, markdown, priority, date, created_at, updated_at FROM notes WHERE id = $1"
        )
        .bind(&created_note.id)
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch created note");

        assert_eq!(fetched_note.id, created_note.id);
        assert_eq!(fetched_note.title, "Test Note");
    }
}