pub use operations::{run_operations, };

#[tauri::command]
pub fn reverse(request: &str) -> Result<String, String> {
    run_operations(, request)
}

