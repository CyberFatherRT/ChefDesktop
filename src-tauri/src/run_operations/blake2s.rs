pub use operations::{run_operations, Blake2s, OutputFormat};

#[tauri::command]
pub fn blake2s(request: &str) -> Result<OutputFormat, String> {
    run_operations(Blake2s, request)
}
