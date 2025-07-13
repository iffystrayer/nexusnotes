use crate::db::init_db;
use crate::models::{new_id, Notebook};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NotebookPayload {
    id: Option<String>,
    parent_id: Option<String>,
    title: String,
    icon: Option<String>,
}

#[tauri::command]
pub async fn get_notebooks() -> Result<Vec<Notebook>, String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    let rows = sqlx::query_as!(
        Notebook,
        "SELECT id, parent_id, title, icon, sort_order FROM notebooks ORDER BY sort_order, created_at"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub async fn create_notebook(payload: NotebookPayload) -> Result<Notebook, String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    let id = payload.id.unwrap_or_else(new_id);
    sqlx::query!(
        "INSERT INTO notebooks (id, parent_id, title, icon, sort_order)
         VALUES ($1, $2, $3, $4, (SELECT COALESCE(MAX(sort_order),0)+1 FROM notebooks WHERE parent_id = $2))",
        id,
        payload.parent_id,
        payload.title,
        payload.icon
    )
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(Notebook {
        id,
        parent_id: payload.parent_id,
        title: payload.title,
        icon: payload.icon,
        sort_order: 0, // will be refreshed on next fetch
    })
}

#[tauri::command]
pub async fn rename_notebook(id: String, title: String) -> Result<(), String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    sqlx::query!(
        "UPDATE notebooks SET title = $1 WHERE id = $2",
        title,
        id
    )
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_notebook(id: String) -> Result<(), String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    // cascades via ON DELETE CASCADE
    sqlx::query!("DELETE FROM notebooks WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn move_notebook(id: String, new_parent_id: Option<String>) -> Result<(), String> {
    let pool = init_db().await.map_err(|e| e.to_string())?;
    sqlx::query!(
        "UPDATE notebooks SET parent_id = $1 WHERE id = $2",
        new_parent_id,
        id
    )
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}
