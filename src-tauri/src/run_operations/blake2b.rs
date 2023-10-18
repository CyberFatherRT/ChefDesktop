pub use operations::{run_operations, Blake2b, OutputFormat};

#[tauri::command]
pub fn blake2b(request: &str) -> Result<OutputFormat, String> {
    run_operations(Blake2b, request)
}
