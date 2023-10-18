pub use operations::{run_operations, Argon2Compare};

#[tauri::command]
pub fn argon2_compare(request: &str) -> Result<String, String> {
    run_operations(Argon2Compare, request)
}

