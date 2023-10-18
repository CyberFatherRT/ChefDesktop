pub use operations::{run_operations, Argon2};

#[tauri::command]
pub fn argon2(request: &str) -> Result<String, String> {
    run_operations(Argon2, request)
}
