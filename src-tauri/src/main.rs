// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod run_operations;

pub use run_operations::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
a1z26_cipher_decode,
a1z26_cipher_encode,
