pub use operations::{run_operations, SHA1};

#[tauri::command]
pub fn sha1(request: &str) -> Result<String, String> {
    run_operations(SHA1, request)
}
