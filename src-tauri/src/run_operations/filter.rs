pub use operations::{run_operations, Filter};

#[tauri::command]
pub fn filter(request: &str) -> Result<String, String> {
    run_operations(Filter, request)
}

