//Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod tauri_commands;

// use std::{env::args, process::exit};
// use nix::unistd::{fork, ForkResult};

use tauri_commands::fs::*;
use tauri_commands::utils::*;

fn main() -> anyhow::Result<()> {
    // let args: Vec<_> = args().collect();
    //
    // if args.len() == 1 {
    //     match unsafe { fork() } {
    //         Ok(ForkResult::Child) => start()?,
    //         Ok(_) => exit(0),
    //         Err(err) => {
    //             eprintln!("{err}");
    //             exit(1);
    //         }
    //     }
    // }

    start()?;

    Ok(())
}

fn start() -> anyhow::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![gsd, save_to_file, read_from_file])
        .run(tauri::generate_context!())?;
    Ok(())
}
