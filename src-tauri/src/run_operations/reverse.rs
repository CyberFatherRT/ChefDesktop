pub use operations::{run_operations, ReverseString};

#[tauri::command]
pub fn reverse(request: &str) -> Result<String, String> {
    run_operations(ReverseString, request)
}
