pub use operations::{run_operations, Add};

#[tauri::command]
pub fn add(request: &str) -> Result<String, String> {
    run_operations(Add, request)
}

