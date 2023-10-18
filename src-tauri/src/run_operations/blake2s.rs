pub use operations::{run_operations, Blake2s};

#[tauri::command]
pub fn blake2s(request: &str) -> Result<String, String> {
    run_operations(Blake2s, request)
}
