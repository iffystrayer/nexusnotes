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

    println!("Running database migrations...");
    sqlx::migrate!() // Defaults to ./migrations relative to crate root
        .run(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;

    // Insert default data if notebooks table is empty (only after migrations)
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM notebooks")
        .fetch_one(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to count notebooks: {}", e))?;

    if count.0 == 0 {
        println!("Inserting sample data...");

        // Create root notebooks
        sqlx::query(
            "INSERT INTO notebooks (id, title, icon, sort_order)
             VALUES
             ('inbox_nb', 'Inbox', '📥', 0),
             ('personal_nb', 'Personal', '👤', 1),
             ('work_nb', 'Work', '💼', 2),
             ('projects_nb', 'Projects', '🚀', 3)"
        )
        .execute(&pool)
        .await?;

        // Create nested notebooks
        sqlx::query(
            "INSERT INTO notebooks (id, parent_id, title, icon, sort_order)
             VALUES
             ('daily_notes', 'personal_nb', 'Daily Notes', '📅', 0),
             ('recipes', 'personal_nb', 'Recipes', '🍳', 1),
             ('meetings', 'work_nb', 'Meetings', '📝', 0),
             ('ideas', 'work_nb', 'Ideas', '💡', 1),
             ('web_app', 'projects_nb', 'Web App', '🌐', 0),
             ('mobile_app', 'projects_nb', 'Mobile App', '📱', 1)"
        )
        .execute(&pool)
        .await?;

        // Create sample notes
        sqlx::query(
            "INSERT INTO notes (id, notebook_id, title, markdown)
             VALUES
             ('note1', 'inbox_nb', 'Welcome to NexusNotes', 'This is your inbox for quick captures!'),
             ('note2', 'daily_notes', 'Today''s Goals', '- [ ] Review project updates\n- [ ] Call mom\n- [ ] Grocery shopping'),
             ('note3', 'daily_notes', 'Meeting Notes - Jan 12', '## Team Standup\n- Alice: Working on frontend\n- Bob: Database optimization'),
             ('note4', 'recipes', 'Chocolate Chip Cookies', '## Ingredients\n- 2 cups flour\n- 1 cup sugar\n- 1/2 cup butter\n\n## Instructions\n1. Preheat oven to 350°F'),
             ('note5', 'meetings', 'Q1 Planning Session', '## Agenda\n1. Review last quarter\n2. Set Q1 goals\n3. Resource allocation'),
             ('note6', 'ideas', 'App Feature Ideas', '- Dark mode toggle\n- Export to PDF\n- Real-time collaboration\n- Mobile app'),
             ('note7', 'web_app', 'Tech Stack Decision', '## Frontend\n- React vs Svelte\n- TypeScript\n\n## Backend\n- Node.js\n- PostgreSQL'),
             ('note8', 'mobile_app', 'UI Mockups', 'Created initial wireframes for:\n- Login screen\n- Dashboard\n- Note editor')"
        )
        .execute(&pool)
        .await?;

        println!("Sample data inserted successfully!");
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
