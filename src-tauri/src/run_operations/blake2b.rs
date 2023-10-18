pub use operations::{run_operations, };

#[tauri::command]
pub fn blake2b(request: &str) -> Result<String, String> {
    run_operations(, request)
}

