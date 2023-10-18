use operations::{run_operations, ToBase64};

#[tauri::command]
pub fn to_base64(request: &str) -> Result<String, String> {
    run_operations(ToBase64, request)
}
