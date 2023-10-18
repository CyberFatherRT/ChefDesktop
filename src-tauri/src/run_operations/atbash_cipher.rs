pub use operations::{run_operations, AtbashCipher};

#[tauri::command]
pub fn atbash_cipher(request: &str) -> Result<String, String> {
    run_operations(AtbashCipher, request)
}

