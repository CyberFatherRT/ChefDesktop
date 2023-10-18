pub use operations::{run_operations, Blake2b};

#[tauri::command]
pub fn blake2b(request: &str) -> Result<String, String> {
    run_operations(Blake2b, request)
}
