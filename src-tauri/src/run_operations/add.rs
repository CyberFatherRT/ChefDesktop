pub use operations::{run_operations, };

#[tauri::command]
pub fn add(request: &str) -> Result<String, String> {
    run_operations(, request)
}

