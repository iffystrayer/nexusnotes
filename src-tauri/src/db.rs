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
