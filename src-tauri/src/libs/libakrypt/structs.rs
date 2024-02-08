use std::ffi::{c_int, c_char, c_uchar};

#[allow(non_camel_case_types)]
pub type ak_uint8 = c_uchar;

#[repr(C)]
pub struct Config {
    algorithm: *const c_char,
    input: *mut c_char,
    output: *mut c_char,
    key: *const ak_uint8,
    key_size: usize,
    iv: *const ak_uint8,
    iv_size: usize,
    mode: Mode,
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
#[derive(Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum Mode {
    CBC,
    CTR,
    OFB,
    CFB,
    ECB,
}

pub enum OutputFormat {
    Hex,
    Base64,
}

pub enum AkryptFunction {
    Kuznechik,
    Magma,
}

pub enum InputFormat {
    Hex,
    Base64,
    Raw,
}

extern "C" {
    pub fn akrypt_encrypt(config: *const Config) -> c_int;
    pub fn akrypt_decrypt(config: *const Config) -> c_int;
}

