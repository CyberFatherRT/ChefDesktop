use operations::{Argon2, run_operations};

#[tauri::command]
pub fn argon2(request: &str) -> Result<String, String> {
    run_operations(Argon2, request)
}
