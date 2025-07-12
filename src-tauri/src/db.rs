use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::PathBuf;

pub async fn init_db() -> Result<SqlitePool> {
    let path = data_dir()?.join("nexusnotes.sqlite");
    println!("Initializing database at: {}", path.display());
    
    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", path.display()))
        .await?;

    // run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Database initialized successfully");
    Ok(pool)
}

fn data_dir() -> Result<PathBuf> {
    let dir = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("no data dir"))?
        .join("nexusnotes");
    std::fs::create_dir_all(&dir)?;
    println!("Data directory: {}", dir.display());
    Ok(dir)
}
