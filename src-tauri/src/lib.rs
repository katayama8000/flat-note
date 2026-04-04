use domain::aggregate::value_object::SortBy;
use domain::Page;
use infrastructure::LibSqlPageRepository;
use usecase::{
    CountPagesUseCase, CreatePageUseCase, GetPageUseCase, GetPagesUseCase, SearchPagesUseCase,
    SuggestPageTitlesUseCase, UpdatePageUseCase, UpdateTitleUseCase,
};
use uuid::Uuid;

const CURRENT_USER_ID: &str = "me-local-001";
const DEFAULT_DB_URL: &str = "file:local.db";

fn db_url() -> String {
    std::env::var("FLAT_NOTE_DB_URL").unwrap_or_else(|_| DEFAULT_DB_URL.to_string())
}

#[tauri::command]
async fn get_pages(sort_by: SortBy) -> Result<Vec<Page>, String> {
    let repository = LibSqlPageRepository::new(db_url());
    let use_case = GetPagesUseCase::new(repository);
    use_case.execute(CURRENT_USER_ID, &sort_by).await
}

#[tauri::command]
async fn get_page(id: String) -> Result<Option<Page>, String> {
    let repository = LibSqlPageRepository::new(db_url());
    let use_case = GetPageUseCase::new(repository);
    use_case.execute(CURRENT_USER_ID, &id).await
}

#[tauri::command]
async fn update_page(id: String, description: String) -> Result<(), String> {
    let repository = LibSqlPageRepository::new(db_url());
    let use_case = UpdatePageUseCase::new(repository);
    use_case.execute(CURRENT_USER_ID, &id, &description).await
}

#[tauri::command]
async fn update_title(id: String, title: String) -> Result<(), String> {
    let repository = LibSqlPageRepository::new(db_url());
    let use_case = UpdateTitleUseCase::new(repository);
    use_case.execute(CURRENT_USER_ID, &id, &title).await
}

#[tauri::command]
async fn update_title_direct(id: String, title: String) -> Result<(), String> {
    let repository = LibSqlPageRepository::new(db_url());
    let use_case = UpdateTitleUseCase::new(repository);
    use_case.execute_direct(CURRENT_USER_ID, &id, &title).await
}

#[tauri::command]
async fn update_page_direct(id: String, description: String) -> Result<(), String> {
    let repository = LibSqlPageRepository::new(db_url());
    let use_case = UpdatePageUseCase::new(repository);
    use_case
        .execute_direct(CURRENT_USER_ID, &id, &description)
        .await
}

#[tauri::command]
async fn create_page(title: String) -> Result<Page, String> {
    let id = Uuid::new_v4().to_string();
    let repository = LibSqlPageRepository::new(db_url());
    let use_case = CreatePageUseCase::new(repository);
    use_case.execute(CURRENT_USER_ID, &id, &title).await
}

#[tauri::command]
async fn get_page_count() -> Result<u64, String> {
    let repository = LibSqlPageRepository::new(db_url());
    let use_case = CountPagesUseCase::new(repository);
    use_case.execute(CURRENT_USER_ID).await
}

#[tauri::command]
async fn search_pages(query: String, sort_by: SortBy, limit: u32) -> Result<Vec<Page>, String> {
    let repository = LibSqlPageRepository::new(db_url());
    let use_case = SearchPagesUseCase::new(repository);
    use_case
        .execute(CURRENT_USER_ID, &query, &sort_by, limit)
        .await
}

#[tauri::command]
async fn suggest_page_titles(query: String, limit: u32) -> Result<Vec<String>, String> {
    let repository = LibSqlPageRepository::new(db_url());
    let use_case = SuggestPageTitlesUseCase::new(repository);
    use_case.execute(CURRENT_USER_ID, &query, limit).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_pages,
            get_page,
            get_page_count,
            search_pages,
            suggest_page_titles,
            update_page,
            update_page_direct,
            update_title,
            update_title_direct,
            create_page
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
