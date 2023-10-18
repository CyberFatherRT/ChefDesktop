pub use operations::{run_operations, };

#[tauri::command]
pub fn analyse_hash(request: &str) -> Result<String, String> {
    run_operations(, request)
}

