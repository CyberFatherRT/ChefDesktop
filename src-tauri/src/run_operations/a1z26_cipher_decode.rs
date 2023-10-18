pub use operations::{run_operations, A1z26CipherDecode};

#[tauri::command]
pub fn a1z26_cipher_decode(request: &str) -> Result<String, String> {
    run_operations(A1z26CipherDecode, request)
}

