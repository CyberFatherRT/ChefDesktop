pub use operations::{run_operations, Hmac};

#[tauri::command]
pub fn hmac(request: &str) -> Result<String, String> {
    run_operations(Hmac, request)
}

