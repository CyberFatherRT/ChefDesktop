pub use operations::{run_operations, Md5};

#[tauri::command]
pub fn md5(request: &str) -> Result<String, String> {
    run_operations(Md5, request)
}
