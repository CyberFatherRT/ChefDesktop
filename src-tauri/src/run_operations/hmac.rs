pub use operations::{run_operations, Hmac, OutputFormat};

#[tauri::command]
pub fn hmac(request: &str) -> Result<OutputFormat, String> {
    run_operations(Hmac, request)
}
