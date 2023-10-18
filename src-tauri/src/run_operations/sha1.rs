pub use operations::{run_operations, Sha1};

#[tauri::command]
pub fn sha1(request: &str) -> Result<String, String> {
    run_operations(Sha1, request)
}
