pub use operations::{run_operations, Sha3};

#[tauri::command]
pub fn sha3(request: &str) -> Result<String, String> {
    run_operations(Sha3, request)
}
