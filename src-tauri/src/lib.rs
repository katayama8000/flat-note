use serde::Serialize;

// Page struct matching the Page type defined in TypeScript
#[derive(Serialize)]
struct Page {
    id: String,
    title: String,
    description: String,
}

// Command callable from TypeScript via invoke("get_pages")
#[tauri::command]
fn get_pages() -> Vec<Page> {
    vec![
        Page {
            id: "1".to_string(),
            title: "Page returned from Rust".to_string(),
            description: "This page was generated in Rust".to_string(),
        },
        Page {
            id: "2".to_string(),
            title: "Second page".to_string(),
            description: "Automatically serialized to JSON via Serialize derive".to_string(),
        },
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_pages])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
