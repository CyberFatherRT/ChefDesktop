pub use operations::{run_operations, RsaDecrypt};

#[tauri::command]
pub fn rsa_decrypt(request: &str) -> Result<String, String> {
    run_operations(RsaDecrypt, request)
}

