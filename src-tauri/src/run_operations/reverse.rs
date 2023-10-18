pub use operations::{run_operations, Reverse};

#[tauri::command]
pub fn reverse(request: &str) -> Result<String, String> {
    run_operations(Reverse, request)
}
