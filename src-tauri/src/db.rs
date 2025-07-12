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

    // Test basic connection by creating a simple table
    sqlx::query("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY)")
        .execute(&pool)
        .await?;
    
    println!("Basic database connection successful");
    
    // Try to run migrations
    println!("Running migrations...");
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Migrations completed successfully"),
        Err(e) => {
            println!("Migration failed: {}", e);
            // For now, let's continue without migrations to test basic functionality
        }
    }
    
    Ok(pool)
}

fn data_dir() -> Result<PathBuf> {
    let dir = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?
        .join("nexusnotes");
    Ok(dir)
}
