pub use operations::{run_operations, AND};

#[tauri::command]
pub fn and(request: &str) -> Result<String, String> {
    run_operations(AND, request)
}
