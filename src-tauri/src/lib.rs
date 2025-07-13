// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod db;
mod models;
mod notebooks;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::async_runtime::spawn(async {
        if let Err(e) = db::init_db().await {
            eprintln!("Failed to initialize database: {}", e);
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            notebooks::get_notebooks,
            notebooks::create_notebook,
            notebooks::delete_notebook,
            notebooks::rename_notebook,
            notebooks::move_notebook
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
