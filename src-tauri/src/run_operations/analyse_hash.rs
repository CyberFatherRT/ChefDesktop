pub use operations::{run_operations, AnalyseHash, AnalyseHashSerializeMeDaddy};

#[tauri::command]
pub fn analyse_hash(request: &str) -> Result<AnalyseHashSerializeMeDaddy, String> {
    run_operations(AnalyseHash, request)
}
