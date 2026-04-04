fn main() {
    // Pass DB credentials through at compile time so they are baked into
    // the released binary and available without shell environment variables.
    if let Ok(url) = std::env::var("FLAT_NOTE_DB_URL") {
        println!("cargo:rustc-env=FLAT_NOTE_DB_URL={url}");
    }
    if let Ok(token) = std::env::var("FLAT_NOTE_DB_AUTH_TOKEN") {
        println!("cargo:rustc-env=FLAT_NOTE_DB_AUTH_TOKEN={token}");
    }

    // Re-run this script whenever either variable changes.
    println!("cargo:rerun-if-env-changed=FLAT_NOTE_DB_URL");
    println!("cargo:rerun-if-env-changed=FLAT_NOTE_DB_AUTH_TOKEN");

    tauri_build::build()
}
