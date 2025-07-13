use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::PathBuf;

pub async fn init_db() -> Result<SqlitePool> {
    let data_dir = data_dir()?;
    let path = data_dir.join("nexusnotes.sqlite");
    println!("Initializing database at: {}", path.display());
    
    // Ensure the parent directory exists and is writable
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)?;
        println!("Created data directory: {}", data_dir.display());
    }
    
    // Test basic connection first
    let database_url = format!("sqlite:{}", path.display());
    println!("Connecting to database: {}", database_url);
    
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;

    // Create tables manually if migrations fail
    println!("Creating tables manually...");
    
    // Create notebooks table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS notebooks (
            id           TEXT PRIMARY KEY,
            parent_id    TEXT REFERENCES notebooks(id) ON DELETE CASCADE,
            title        TEXT NOT NULL,
            icon         TEXT,
            sort_order   INTEGER DEFAULT 0,
            created_at   DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;
    
    // Create notes table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS notes (
            id           TEXT PRIMARY KEY,
            notebook_id  TEXT NOT NULL REFERENCES notebooks(id) ON DELETE CASCADE,
            title        TEXT NOT NULL,
            markdown     TEXT NOT NULL DEFAULT '',
            priority     INTEGER DEFAULT 0,
            date         DATE,
            created_at   DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at   DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;
    
    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_notes_notebook ON notes(notebook_id)")
        .execute(&pool)
        .await?;
    
    // Create tags table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tags (
            id   TEXT PRIMARY KEY,
            name TEXT UNIQUE NOT NULL
        )"
    )
    .execute(&pool)
    .await?;
    
    // Create note_tags table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS note_tags (
            note_id TEXT REFERENCES notes(id) ON DELETE CASCADE,
            tag_id  TEXT REFERENCES tags(id)   ON DELETE CASCADE,
            PRIMARY KEY (note_id, tag_id)
        )"
    )
    .execute(&pool)
    .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_note_tags_note ON note_tags(note_id)")
        .execute(&pool)
        .await?;
    
    // Create versions table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS versions (
            id         TEXT PRIMARY KEY,
            note_id    TEXT NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
            markdown   TEXT NOT NULL,
            saved_at   DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;
    
    // Insert default data if tables are empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM notebooks")
        .fetch_one(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to count notebooks: {}", e))?;
    
    if count.0 == 0 {
        println!("Inserting default data...");
        // Insert default Inbox notebook
        sqlx::query(
            "INSERT INTO notebooks (id, title, icon, sort_order)
             VALUES ('inbox_nb', 'Inbox', '📥', 0)"
        )
        .execute(&pool)
        .await?;
        
        // Insert default Inbox note
        sqlx::query(
            "INSERT INTO notes (id, notebook_id, title, markdown)
             VALUES ('inbox', 'inbox_nb', 'Inbox', 'Quick-capture goes here…')"
        )
        .execute(&pool)
        .await?;
    }
    
    println!("Database initialized successfully");
    
    Ok(pool)
}

fn data_dir() -> Result<PathBuf> {
    let dir = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?
        .join("nexusnotes");
    Ok(dir)
}
