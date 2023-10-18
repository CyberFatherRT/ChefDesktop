pub use operations::{run_operations, };

#[tauri::command]
pub fn md4(request: &str) -> Result<String, String> {
    run_operations(, request)
}

