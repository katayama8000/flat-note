use domain::Page;
use infrastructure::LibSqlPageRepository;
use usecase::GetPagesUseCase;

const DB_URL: &str = "http://127.0.0.1:8080";

// Command callable from TypeScript via invoke("get_pages")
#[tauri::command]
async fn get_pages() -> Result<Vec<Page>, String> {
    let repository = LibSqlPageRepository::new(DB_URL);
    let use_case = GetPagesUseCase::new(repository);
    use_case.execute().await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_pages])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
