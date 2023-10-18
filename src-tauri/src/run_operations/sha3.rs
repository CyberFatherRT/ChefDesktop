pub use operations::{run_operations, SHA3};

#[tauri::command]
pub fn sha3(request: &str) -> Result<String, String> {
    run_operations(SHA3, request)
}
