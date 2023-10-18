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
add_line_numbers,
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
