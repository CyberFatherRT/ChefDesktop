pub use operations::{run_operations, MD2};

#[tauri::command]
pub fn md2(request: &str) -> Result<String, String> {
    run_operations(MD2, request)
}
