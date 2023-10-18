pub use operations::{run_operations, And};

#[tauri::command]
pub fn and(request: &str) -> Result<String, String> {
    run_operations(And, request)
}
