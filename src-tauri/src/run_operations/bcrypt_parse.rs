pub use operations::{run_operations, BcryptParse, BcryptParseHashParts};

#[tauri::command]
pub fn bcrypt_parse(request: &str) -> Result<BcryptParseHashParts, String> {
    run_operations(BcryptParse, request)
}
