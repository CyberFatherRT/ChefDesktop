pub use operations::{run_operations, };

#[tauri::command]
pub fn rsa_encrypt(request: &str) -> Result<String, String> {
    run_operations(, request)
}

