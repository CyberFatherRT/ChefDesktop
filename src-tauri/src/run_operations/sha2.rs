pub use operations::{run_operations, };

#[tauri::command]
pub fn sha2(request: &str) -> Result<String, String> {
    run_operations(, request)
}

