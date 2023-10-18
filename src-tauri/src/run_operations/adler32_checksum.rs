pub use operations::{run_operations, Adler32CheckSum};

#[tauri::command]
pub fn adler32_checksum(request: &str) -> Result<String, String> {
    run_operations(Adler32CheckSum, request)
}
