pub use operations::{run_operations, AffineCipherEncode};

#[tauri::command]
pub fn affine_cipher_encode(request: &str) -> Result<String, String> {
    run_operations(AffineCipherEncode, request)
}
