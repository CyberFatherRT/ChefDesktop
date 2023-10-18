pub use operations::{run_operations, };

#[tauri::command]
pub fn bcrypt_compare(request: &str) -> Result<String, String> {
    run_operations(, request)
}

