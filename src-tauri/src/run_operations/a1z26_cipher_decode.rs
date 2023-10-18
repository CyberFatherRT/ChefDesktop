pub use operations::{run_operations, };

#[tauri::command]
pub fn a1z26_cipher_decode(request: &str) -> Result<String, String> {
    run_operations(, request)
}

