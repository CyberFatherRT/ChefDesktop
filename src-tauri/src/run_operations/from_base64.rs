pub use operations::{run_operations, };

#[tauri::command]
pub fn from_base64(request: &str) -> Result<String, String> {
    run_operations(, request)
}

