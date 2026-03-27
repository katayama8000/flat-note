use domain::Page;
use infrastructure::LibSqlPageRepository;
use usecase::{GetPageUseCase, GetPagesUseCase, UpdatePageUseCase};

const DB_URL: &str = "http://127.0.0.1:8080";

#[tauri::command]
async fn get_pages() -> Result<Vec<Page>, String> {
    let repository = LibSqlPageRepository::new(DB_URL);
    let use_case = GetPagesUseCase::new(repository);
    use_case.execute().await
}

#[tauri::command]
async fn get_page(id: String) -> Result<Option<Page>, String> {
    let repository = LibSqlPageRepository::new(DB_URL);
    let use_case = GetPageUseCase::new(repository);
    use_case.execute(&id).await
}

#[tauri::command]
async fn update_page(id: String, description: String) -> Result<(), String> {
    let repository = LibSqlPageRepository::new(DB_URL);
    let use_case = UpdatePageUseCase::new(repository);
    use_case.execute(&id, &description).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_pages, get_page, update_page])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
