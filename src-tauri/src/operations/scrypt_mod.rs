use crate::{
    run_op,
    utils::{convert_to_byte_array, SupportedFormats},
    Operation,
};
use anyhow::Result;
use scrypt::{password_hash::PasswordHasher, Scrypt as MScrypt};
use scrypt::{password_hash::SaltString, Params};
use serde::{Deserialize, Serialize};

run_op!(run_scrypt, Scrypt);

impl Operation<'_, DeserializeMeDaddy> for Scrypt {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let DeserializeMeDaddy {
            salt,
            salt_format,
            iterations,
            memory,
            parallelism,
            key_length,
        } = request;

        let salt = convert_to_byte_array(&salt, &salt_format)?;
        let salt = SaltString::encode_b64(&salt)?;
        let params = Params::new(memory, iterations, parallelism, key_length)?;
        let password_hash =
            MScrypt.hash_password_customized(input.as_bytes(), None, None, params, &salt)?;

        Ok(password_hash.to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct DeserializeMeDaddy {
    salt: String,
    salt_format: SupportedFormats,
    iterations: u32,
    memory: u8,
    parallelism: u32,
    key_length: usize,
}

#[derive(Serialize, Deserialize)]
enum SaltFormats {
    Hex,
    Base64,
    UTF8,
    Latin1,
}

pub struct Scrypt;
