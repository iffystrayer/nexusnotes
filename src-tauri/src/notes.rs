use crate::db::init_db;
use crate::models::{new_id, Note};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NotePayload {
    title: String,
    notebook_id: String,
    markdown: String,
}

#[tauri::command]
pub async fn get_notes(notebook_id: String) -> Result<Vec<Note>, String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    let rows = sqlx::query_as::<_, Note>(
        "SELECT id, notebook_id, title, markdown, priority, date, created_at, updated_at
         FROM notes WHERE notebook_id = $1 ORDER BY updated_at DESC"
    )
    .bind(&notebook_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub async fn create_note(payload: NotePayload) -> Result<Note, String> {
    if payload.title.trim().is_empty() {
        return Err("Note title cannot be empty.".into());
    }
    let pool = init_db().await.map_err(|e| e.to_string())?;
    let id = new_id();
    sqlx::query(
        "INSERT INTO notes (id, notebook_id, title, markdown)
         VALUES ($1, $2, $3, $4)"
    )
    .bind(&id)
    .bind(&payload.notebook_id)
    .bind(&payload.title)
    .bind(&payload.markdown)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    // Fetch the newly created note to get accurate timestamps
    let note = sqlx::query_as::<_, Note>(
        "SELECT id, notebook_id, title, markdown, priority, date, created_at, updated_at
         FROM notes WHERE id = $1"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(note)
}

#[tauri::command]
pub async fn update_note(
    id: String,
    title: String,
    markdown: String,
    priority: i32,
    date: Option<String>,
) -> Result<(), String> {
    if title.trim().is_empty() {
        return Err("Note title cannot be empty.".into());
    }
    let pool = init_db().await.map_err(|e| e.to_string())?;
    sqlx::query(
        "UPDATE notes SET title = $1, markdown = $2, priority = $3, date = $4, updated_at = CURRENT_TIMESTAMP WHERE id = $5"
    )
    .bind(&title)
    .bind(&markdown)
    .bind(&priority)
    .bind(&date)
    .bind(&id)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_backlinks(note_id: String) -> Result<Vec<Note>, String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    let search_pattern = format!("%{}%", note_id); // Search for the note ID in markdown

    let notes = sqlx::query_as::<_, Note>(
        "SELECT id, notebook_id, title, markdown, priority, date, created_at, updated_at
         FROM notes WHERE LOWER(markdown) LIKE $1 AND id != $2"
    )
    .bind(&search_pattern)
    .bind(&note_id) // Exclude the note itself from backlinks
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(notes)
}

#[tauri::command]
pub async fn search_notes_and_notebooks(query: String) -> Result<Vec<crate::models::SearchResult>, String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    let search_query = format!("%{}%", query.to_lowercase());

    let notes = sqlx::query_as::<_, crate::models::Note>(
        "SELECT id, notebook_id, title, markdown, priority, date, created_at, updated_at
         FROM notes WHERE LOWER(title) LIKE $1 OR LOWER(markdown) LIKE $1"
    )
    .bind(&search_query)
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let notebooks = sqlx::query_as::<_, crate::models::Notebook>(
        "SELECT id, parent_id, title, icon, sort_order FROM notebooks WHERE LOWER(title) LIKE $1"
    )
    .bind(&search_query)
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    // Search for notes by tag name
    let notes_by_tag = sqlx::query_as::<_, crate::models::Note>(
        "SELECT DISTINCT n.id, n.notebook_id, n.title, n.markdown, n.priority, n.date, n.created_at, n.updated_at
         FROM notes n
         JOIN note_tags nt ON n.id = nt.note_id
         JOIN tags t ON nt.tag_id = t.id
         WHERE LOWER(t.name) LIKE $1"
    )
    .bind(&search_query)
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut results: Vec<crate::models::SearchResult> = Vec::new();

    for note in notes {
        results.push(crate::models::SearchResult::Note(note));
    }
    for notebook in notebooks {
        results.push(crate::models::SearchResult::Notebook(notebook));
    }
    for note in notes_by_tag {
        // Avoid duplicates if a note was already found by title/markdown
        if !results.iter().any(|r| match r {
            crate::models::SearchResult::Note(n) => n.id == note.id,
            _ => false,
        }) {
            results.push(crate::models::SearchResult::Note(note));
        }
    }

    Ok(results)
}

#[tauri::command]
pub async fn get_tags_for_note(note_id: String) -> Result<Vec<crate::models::Tag>, String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    let tags = sqlx::query_as::<_, crate::models::Tag>(
        "SELECT t.id, t.name FROM tags t JOIN note_tags nt ON t.id = nt.tag_id WHERE nt.note_id = $1"
    )
    .bind(&note_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(tags)
}

#[tauri::command]
pub async fn add_tag_to_note(note_id: String, tag_name: String) -> Result<(), String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    let tag_id = new_id();

    // Check if tag exists, if not, create it
    let existing_tag_id: Option<String> = sqlx::query_scalar("SELECT id FROM tags WHERE name = $1")
        .bind(&tag_name)
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let final_tag_id = if let Some(id) = existing_tag_id {
        id
    } else {
        sqlx::query("INSERT INTO tags (id, name) VALUES ($1, $2)")
            .bind(&tag_id)
            .bind(&tag_name)
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
        tag_id
    };

    // Add note_tag entry
    sqlx::query("INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES ($1, $2)")
        .bind(&note_id)
        .bind(&final_tag_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn remove_tag_from_note(note_id: String, tag_name: String) -> Result<(), String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    sqlx::query(
        "DELETE FROM note_tags WHERE note_id = $1 AND tag_id = (SELECT id FROM tags WHERE name = $2)"
    )
    .bind(&note_id)
    .bind(&tag_name)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_all_tags() -> Result<Vec<crate::models::Tag>, String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    let tags = sqlx::query_as::<_, crate::models::Tag>(
        "SELECT id, name FROM tags ORDER BY name"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(tags)
}

#[tauri::command]
pub async fn delete_note(id: String) -> Result<(), String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM notes WHERE id = $1")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
