pub use operations::{run_operations, Sha2};

#[tauri::command]
pub fn sha2(request: &str) -> Result<String, String> {
    run_operations(Sha2, request)
}

