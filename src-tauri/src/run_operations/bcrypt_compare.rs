pub use operations::{run_operations, BcryptCompare};

#[tauri::command]
pub fn bcrypt_compare(request: &str) -> Result<String, String> {
    run_operations(BcryptCompare, request)
}

