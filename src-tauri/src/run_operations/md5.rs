pub use operations::{run_operations, MD5};

#[tauri::command]
pub fn md5(request: &str) -> Result<String, String> {
    run_operations(MD5, request)
}
