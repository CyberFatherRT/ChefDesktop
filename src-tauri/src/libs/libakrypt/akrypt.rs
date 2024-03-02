use std::ffi::{c_char, CString};

use crate::{
    libs::base64::{from_base64, to_base64},
    utils::from_hex,
};
use anyhow::{bail, Context, Result};

use super::structs::{
    ak_uint8, akrypt_decrypt, akrypt_encrypt, AkryptFunction, Config, InputFormat, Mode,
    OutputFormat,
};

#[repr(C)]
pub struct Akrypt {
    algorithm: AkryptFunction,
    input: CString,
    output: String,
    key: Vec<ak_uint8>,
    key_size: usize,
    iv: Vec<ak_uint8>,
    iv_size: usize,
    mode: Mode,
}

impl Akrypt {
    pub fn new(algorithm: AkryptFunction) -> Self {
        Self {
            algorithm,
            input: CString::new("").unwrap(),
            output: String::new(),
            key: Vec::new(),
            key_size: 0,
            iv: Vec::new(),
            iv_size: 0,
            mode: Mode::CBC,
        }
    }

    pub fn encrypt(&self, format: OutputFormat) -> Result<String> {
        let output = vec![0; self.input.to_bytes().len() * 2];

        let config = Config::new(
            match self.algorithm {
                AkryptFunction::Kuznechik => CString::new("kuznyechik").unwrap().into_raw(),
                AkryptFunction::Magma => CString::new("magma").unwrap().into_raw(),
            },
            self.input.as_ptr() as *mut c_char,
            output.as_ptr() as *mut c_char,
            &self.key,
            &self.iv,
            self.mode,
        );

        let return_code = unsafe { akrypt_encrypt(&config as *const Config) };
        if return_code != 0 {
            bail!("Some error in C code");
        }

        let output = String::from_utf8_lossy(&output)
            .trim_end_matches('\0')
            .to_string();

        let output = match format {
            OutputFormat::Hex => output,
            OutputFormat::Base64 => to_base64(&from_hex(&output)?),
            OutputFormat::Raw => String::from_utf8_lossy(&from_hex(&output)?).to_string(),
        };

        Ok(output)
    }

    pub fn decrypt(&self) -> Result<String> {
        let output = vec![0; self.input.to_bytes().len()];

        let config = Config::new(
            match self.algorithm {
                AkryptFunction::Kuznechik => CString::new("kuznyechik").unwrap().into_raw(),
                AkryptFunction::Magma => CString::new("magma").unwrap().into_raw(),
            },
            self.input.as_ptr() as *mut c_char,
            output.as_ptr() as *mut c_char,
            &self.key,
            &self.iv,
            self.mode,
        );

        let return_code = unsafe { akrypt_decrypt(&config as *const Config) };
        if return_code != 0 {
            bail!("Some error in C code");
        }

        let output = Akrypt::unpad(&output)?;

        Ok(String::from_utf8_lossy(&output)
            .trim_end_matches(char::from(0))
            .to_string())
    }

    pub fn set_mode(&mut self, mode: Mode) -> &mut Self {
        self.mode = mode;
        self
    }

    pub fn set_input(&mut self, input: &str, format: InputFormat) -> Result<&mut Self> {
        let input = match format {
            InputFormat::Hex => hex::decode(input)?,
            InputFormat::Base64 => from_base64(input)?,
            InputFormat::Raw => input.as_bytes().to_vec(),
        };

        let input = match self.algorithm {
            AkryptFunction::Kuznechik => Akrypt::pad(&input, 32),
            AkryptFunction::Magma => Akrypt::pad(&input, 16),
        };

        self.input = CString::new(input)?;
        Ok(self)
    }

    pub fn set_key(&mut self, key: &[ak_uint8]) -> Result<&mut Self> {
        let key = Akrypt::pad(key, 32);
        let key_size = key.len();

        self.key = key;
        self.key_size = key_size;

        Ok(self)
    }

    pub fn set_iv(&mut self, iv: &[ak_uint8]) -> Result<&mut Self> {
        let iv = match self.algorithm {
            AkryptFunction::Kuznechik => Akrypt::pad(iv, 16),
            AkryptFunction::Magma => Akrypt::pad(iv, 8),
        };
        let iv_size = iv.len();

        self.iv = iv;
        self.iv_size = iv_size;

        Ok(self)
    }

    fn pad(input: &[ak_uint8], size: usize) -> Vec<ak_uint8> {
        let padding = size - (input.len() % size);
        let output = [
            input.to_vec().as_slice(),
            &vec![padding as ak_uint8; padding].as_slice(),
        ]
        .concat();

        if output.len() != size
            && output[output.len() - size..] == vec![padding as ak_uint8; padding]
        {
            return output[..output.len() - size].to_vec();
        }

        output
    }

    fn unpad(input: &[ak_uint8]) -> Result<Vec<u8>> {
        let last_byte = *input.last().context("Value is empty")? as usize;
        if last_byte <= input.len()
            && input[input.len() - last_byte..] == vec![last_byte as u8; last_byte]
        {
            return Ok(input[..input.len() - last_byte].to_vec());
        }
        Ok(input.to_vec())
    }
}
