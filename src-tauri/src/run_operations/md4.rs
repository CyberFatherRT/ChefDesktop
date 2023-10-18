pub use operations::{run_operations, MD4};

#[tauri::command]
pub fn md4(request: &str) -> Result<String, String> {
    run_operations(MD4, request)
}
