pub use operations::{run_operations, Bcrypt};

#[tauri::command]
pub fn bcrypt(request: &str) -> Result<String, String> {
    run_operations(Bcrypt, request)
}

