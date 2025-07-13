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
    
    Ok(Note {
        id,
        notebook_id: payload.notebook_id,
        title: payload.title,
        markdown: payload.markdown,
        priority: 0,
        date: None,
        created_at: "".into(),
        updated_at: "".into(),
    })
}

#[tauri::command]
pub async fn update_note(id: String, title: String, markdown: String) -> Result<(), String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE notes SET title = $1, markdown = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3")
        .bind(&title)
        .bind(&markdown)
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
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
