use operations::{run_operations, A1Z26CipherEncode};

#[tauri::command]
pub fn a1z26_cipher_encode(request: &str) -> Result<String, String> {
    run_operations(A1Z26CipherEncode, request)
}
