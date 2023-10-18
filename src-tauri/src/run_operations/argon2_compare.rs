pub use operations::{run_operations, };

#[tauri::command]
pub fn argon2_compare(request: &str) -> Result<String, String> {
    run_operations(, request)
}

