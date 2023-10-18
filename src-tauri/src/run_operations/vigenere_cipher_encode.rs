pub use operations::{run_operations, VigenereCipherEncode};

#[tauri::command]
pub fn vigenere_cipher_encode(request: &str) -> Result<String, String> {
    run_operations(VigenereCipherEncode, request)
}

