pub use operations::{run_operations, };

#[tauri::command]
pub fn to_base64(request: &str) -> Result<String, String> {
    run_operations(, request)
}

