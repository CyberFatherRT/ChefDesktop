pub use operations::{run_operations, };

#[tauri::command]
pub fn affine_cipher_encode(request: &str) -> Result<String, String> {
    run_operations(, request)
}

