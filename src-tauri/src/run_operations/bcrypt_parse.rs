pub use operations::{run_operations, };

#[tauri::command]
pub fn bcrypt_parse(request: &str) -> Result<String, String> {
    run_operations(, request)
}

