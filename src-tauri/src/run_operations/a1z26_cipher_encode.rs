pub use operations::{run_operations, A1z26CipherEncode};

#[tauri::command]
pub fn a1z26_cipher_encode(request: &str) -> Result<String, String> {
    run_operations(A1z26CipherEncode, request)
}

