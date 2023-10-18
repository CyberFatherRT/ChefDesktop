pub use operations::{run_operations, Md4};

#[tauri::command]
pub fn md4(request: &str) -> Result<String, String> {
    run_operations(Md4, request)
}

