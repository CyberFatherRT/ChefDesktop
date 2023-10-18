pub use operations::{run_operations, };

#[tauri::command]
pub fn adler32_checksum(request: &str) -> Result<String, String> {
    run_operations(, request)
}

