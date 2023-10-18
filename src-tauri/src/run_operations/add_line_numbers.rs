pub use operations::{run_operations, AddLineNumbers};

#[tauri::command]
pub fn add_line_numbers(request: &str) -> Result<String, String> {
    run_operations(AddLineNumbers, request)
}
