pub use operations::{run_operations, };

#[tauri::command]
pub fn hmac(request: &str) -> Result<String, String> {
    run_operations(, request)
}

