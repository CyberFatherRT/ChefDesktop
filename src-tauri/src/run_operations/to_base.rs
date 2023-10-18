pub use operations::{run_operations, ToBase};

#[tauri::command]
pub fn to_base(request: &str) -> Result<String, String> {
    run_operations(ToBase, request)
}
