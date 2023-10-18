pub use operations::{run_operations, AffineCipherDecode};

#[tauri::command]
pub fn affine_cipher_decode(request: &str) -> Result<String, String> {
    run_operations(AffineCipherDecode, request)
}
