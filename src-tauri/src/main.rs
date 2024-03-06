//Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod tauri_commands;

// use std::{env::args, process::exit};
// use nix::unistd::{fork, ForkResult};

use chef_desktop::*;
use tauri_commands::fs::*;

fn main() -> anyhow::Result<()> {
    // let args: Vec<_> = args().collect();
    //
    // if args.len() == 1 {
    //     match unsafe{fork()} {
    //         Ok(ForkResult::Child) => start()?,
    //         Ok(_) => exit(0),
    //         Err(err) => {
    //             eprintln!("{err}");
    //             exit(1);
    //         },
    //     }
    // }
    //
    // println!("args: {:?}", args);

    start()?;

    Ok(())
}

fn start() -> anyhow::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_to_file,
            read_from_file,
            a1z26_cipher_decode,
            a1z26_cipher_encode,
            add_line_number,
            add,
            adler32_checksum,
            affine_cipher_decode,
            affine_cipher_encode,
            analyse_hash,
            and,
            argon2_compare,
            argon2,
            atbash_cipher,
            bacon_cipher_decode,
            bacon_cipher_encode,
            bcrypt_compare,
            bcrypt,
            bcrypt_parse,
            bifid_cipher_encode,
            blake2b,
            blake2s,
            filter,
            from_base64,
            from_base,
            hmac,
            kuznechik_encrypt,
            kuznechik_decrypt,
            magma_encrypt,
            magma_decrypt,
            md2,
            md4,
            md5,
            reverse,
            rsa_decrypt,
            rsa_encrypt,
            sha1,
            sha2,
            sha3,
            to_base64,
            to_base,
            vigenere_cipher_decode,
            vigenere_cipher_encode,
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
