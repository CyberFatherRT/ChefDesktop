use std::collections::HashMap;

use chef_desktop::Operations;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RecipeOperations {
    name: Operations,
}

#[allow(unused)]
struct BaseOperation {
    name: String,
    op_name: String,
    // module: Modules,
    english_description: String,
    russian_description: String,
    // args: Vec<Arg>,
    params: HashMap<String, String>,
    is_disabled: bool,
    is_breakpoint: bool,
}

#[tauri::command]
#[allow(unused)]
pub fn gsd(input: String, ops: Vec<RecipeOperations>) -> String {
    input
}
