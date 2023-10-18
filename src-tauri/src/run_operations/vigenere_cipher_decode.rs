pub use operations::{run_operations, VigenereCipherDecode};

#[tauri::command]
pub fn vigenere_cipher_decode(request: &str) -> Result<String, String> {
    run_operations(VigenereCipherDecode, request)
}

