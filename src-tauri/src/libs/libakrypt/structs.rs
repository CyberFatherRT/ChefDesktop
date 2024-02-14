use std::ffi::{c_char, c_int, c_uchar};

use serde::Deserialize;

#[allow(non_camel_case_types)]
pub type ak_uint8 = c_uchar;

#[repr(C)]
pub struct Config {
    pub algorithm: *const c_char,
    pub input: *mut c_char,
    pub output: *mut c_char,
    pub key: *const ak_uint8,
    pub key_size: usize,
    pub iv: *const ak_uint8,
    pub iv_size: usize,
    pub mode: Mode,
}

impl Config {
    pub fn new(
        algorithm: *const c_char,
        input: *mut c_char,
        output: *mut c_char,
        key: &[ak_uint8],
        iv: &[ak_uint8],
        mode: Mode,
    ) -> Self {
        Self {
            algorithm,
            input,
            output,
            key: key.as_ptr() as *const ak_uint8,
            key_size: key.len(),
            iv: iv.as_ptr() as *const ak_uint8,
            iv_size: iv.len(),
            mode,
        }
    }
}

#[repr(C)]
#[derive(Deserialize, Copy, Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Mode {
    CBC,
    CTR,
    OFB,
    CFB,
    ECB,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Hex,
    Base64,
    Raw,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum InputFormat {
    Hex,
    Base64,
    Raw,
}

#[derive(Debug)]
pub enum AkryptFunction {
    Kuznechik,
    Magma,
}

extern "C" {
    pub fn akrypt_encrypt(config: *const Config) -> c_int;
    pub fn akrypt_decrypt(config: *const Config) -> c_int;
}
