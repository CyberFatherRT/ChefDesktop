pub use operations::{run_operations, Md2};

#[tauri::command]
pub fn md2(request: &str) -> Result<String, String> {
    run_operations(Md2, request)
}
