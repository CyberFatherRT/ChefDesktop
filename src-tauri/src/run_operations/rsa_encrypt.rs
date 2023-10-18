pub use operations::{run_operations, RsaEncrypt};

#[tauri::command]
pub fn rsa_encrypt(request: &str) -> Result<String, String> {
    run_operations(RsaEncrypt, request)
}

