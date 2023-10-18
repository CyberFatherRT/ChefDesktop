pub use operations::{run_operations, BaconCipherDecode};

#[tauri::command]
pub fn bacon_cipher_decode(request: &str) -> Result<String, String> {
    run_operations(BaconCipherDecode, request)
}
