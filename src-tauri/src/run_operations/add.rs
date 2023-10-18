pub use operations::{run_operations, ADD};

#[tauri::command]
pub fn add(request: &str) -> Result<String, String> {
    run_operations(ADD, request)
}
