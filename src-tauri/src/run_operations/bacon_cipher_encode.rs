pub use operations::{run_operations, BaconCipherEncode};

#[tauri::command]
pub fn bacon_cipher_encode(request: &str) -> Result<String, String> {
    run_operations(BaconCipherEncode, request)
}

