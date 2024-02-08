use std::ffi::{c_char, CString};

use anyhow::{bail, Context, Result};
use base64::Engine;

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

        let output = match format {
            OutputFormat::Hex => String::from_utf8_lossy(&output)
                .trim_end_matches(char::from(0))
                .to_string(),
            OutputFormat::Base64 => {
                let output = String::from_utf8_lossy(&output)
                    .trim_end_matches(char::from(0))
                    .to_string();
                let output = hex::decode(format!(
                    "{output:0>fill$}",
                    fill = output.len() + output.len() % 2
                ))?;
                Akrypt::to_base64(&output)
            }
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

    pub fn set_mode(self, mode: Mode) -> Self {
        Self { mode, ..self }
    }

    pub fn set_input(self, input: &str, format: InputFormat) -> Result<Self> {
        let input = match format {
            InputFormat::Hex => hex::decode(input)?,
            InputFormat::Base64 => Akrypt::from_base64(input)?,
            InputFormat::Raw => input.as_bytes().to_vec(),
        };

        let input = match self.algorithm {
            AkryptFunction::Kuznechik => Akrypt::pad(&input, 32),
            AkryptFunction::Magma => Akrypt::pad(&input, 16),
        };

        Ok(Self {
            input: CString::new(input)?,
            ..self
        })
    }

    pub fn set_key(self, key: &[ak_uint8]) -> Result<Self> {
        let key = Akrypt::pad(key, 32);
        let key_size = key.len();

        Ok(Self {
            key,
            key_size,
            ..self
        })
    }

    pub fn set_iv(self, iv: &[ak_uint8]) -> Result<Self> {
        let iv = match self.algorithm {
            AkryptFunction::Kuznechik => Akrypt::pad(iv, 16),
            AkryptFunction::Magma => Akrypt::pad(iv, 8),
        };
        let iv_size = iv.len();

        Ok(Self {
            iv,
            iv_size,
            ..self
        })
    }

    fn pad(input: &[ak_uint8], size: usize) -> Vec<ak_uint8> {
        let padding = size - (input.len() % size);
        let output = [
            input.to_vec().as_slice(),
            &vec![padding as ak_uint8; padding].as_slice(),
        ]
        .concat();

        if output[output.len() - size..] == vec![padding as ak_uint8; padding] {
            return output[..output.len() - size].to_vec();
        }
        output
    }

    fn unpad(input: &[ak_uint8]) -> Result<Vec<u8>> {
        let last_byte = input.last().context("Value is empty")?;
        if input[input.len() - *last_byte as usize..] == vec![*last_byte; *last_byte as usize] {
            return Ok(input[..input.len() - *last_byte as usize].to_vec());
        }
        Ok(input.to_vec())
    }

    fn from_base64(input: &str) -> Result<Vec<u8>> {
        Ok(base64::prelude::BASE64_STANDARD.decode(input)?)
    }

    fn to_base64(input: &[u8]) -> String {
        base64::prelude::BASE64_STANDARD.encode(input)
    }
}
