pub use operations::{run_operations, };

#[tauri::command]
pub fn vigenere_cipher_decode(request: &str) -> Result<String, String> {
    run_operations(, request)
}

