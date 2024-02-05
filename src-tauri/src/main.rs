//Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

extern crate libc;
use chef_desktop::*;

use std::ffi::{c_char, CStr};

#[link(name="akrypt", kind="static")]
#[link(name="akrypt-base", kind="static")]
extern "C" {
    pub fn foo() -> *const c_char;
}

#[tauri::command]
fn rust_foo() -> Result<String, String> {
    let output = unsafe { foo() };
    let c_str = unsafe { CStr::from_ptr(output) };
    let string = c_str.to_str().map_err(|err| err.to_string())?;
    Ok(string.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            rust_foo,
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
