pub use operations::{run_operations, OutputFormat, RSADecrypt};

#[tauri::command]
pub fn rsa_decrypt(request: &str) -> Result<OutputFormat, String> {
    run_operations(RSADecrypt, request)
}
