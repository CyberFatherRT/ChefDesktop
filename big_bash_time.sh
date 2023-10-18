#!/usr/bin/env bash

operation_path="$HOME/personal/projects/chef/ChefApi_operations/src/operations/"
tauri_operation_path="$HOME/personal/projects/chef/ChefDesktop/src-tauri/src/run_operations/"
tauri_main_path="$HOME/personal/projects/chef/ChefDesktop/src-tauri/src/main.rs"

cat << EOF > "$tauri_main_path"
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod run_operations;

pub use run_operations::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
EOF

for f in $(ls "$operation_path"); do
  file_name=$(echo "$f" | sed "s/_mod\.rs//g")
  operation_name=$(grep -E "> for " <(cat "$operation_path$f")  | cut -f6 -d " ")
  if [ "$file_name" = "mod.rs" ]; then continue; fi

    cat << EOF >> "$tauri_operation_path/mod.rs"
mod $file_name;
pub use $file_name::$file_name;
EOF

  cat << EOF > "$tauri_operation_path/$file_name.rs"
pub use operations::{run_operations, $operation_name};

#[tauri::command]
pub fn $file_name(request: &str) -> Result<String, String> {
    run_operations($operation_name, request)
}

EOF

  cat << EOF >> "$tauri_main_path"
$file_name,
EOF

#  git add -A
#  git commit -m "update $file_name operation"

done

cat << EOF >> "$tauri_main_path"
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
EOF

cargo fmt --manifest-path "$HOME/personal/projects/chef/ChefDesktop/src-tauri/Cargo.toml"
