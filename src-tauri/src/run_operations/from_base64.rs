pub use operations::{run_operations, FromBase64};

#[tauri::command]
pub fn from_base64(request: &str) -> Result<String, String> {
    run_operations(FromBase64, request)
}

