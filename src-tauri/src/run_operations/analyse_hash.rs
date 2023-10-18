pub use operations::{run_operations, AnalyseHash};

#[tauri::command]
pub fn analyse_hash(request: &str) -> Result<String, String> {
    run_operations(AnalyseHash, request)
}

