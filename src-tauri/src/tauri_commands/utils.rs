use chef_desktop::Operations;
use serde::{Deserialize, Serialize};

#[allow(unused)]
pub struct PrevOps {
    ops: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeOperations {
    name: Operations,
    request: String,
}

#[tauri::command]
pub fn gsd(input: String, ops: Vec<RecipeOperations>) -> String {
    println!("input: {:?}", input);
    println!("ops: {:?}", ops);
    input
}
