pub use operations::{run_operations, OutputFormat, RSAEncrypt};

#[tauri::command]
pub fn rsa_encrypt(request: &str) -> Result<OutputFormat, String> {
    run_operations(RSAEncrypt, request)
}
