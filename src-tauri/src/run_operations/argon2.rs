pub use operations::{run_operations, };

#[tauri::command]
pub fn argon2(request: &str) -> Result<String, String> {
    run_operations(, request)
}

