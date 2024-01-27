use crate::{
    create_me_daddy,
    utils::{convert_to_byte_array, SupportedFormats},
    Operation,
};
use anyhow::Result;
use scrypt::password_hash::SaltString;
use scrypt::{password_hash::PasswordHasher, Scrypt as MScrypt};
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy> for Scrypt {
    fn do_black_magic(&self, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let (
            input,
            Params {
                salt,
                salt_format,
                iterations,
                memory,
                parallelism,
                key_length,
            },
        ) = (request.input, request.params);

        let salt = convert_to_byte_array(&salt, &salt_format)?;
        let salt = SaltString::encode_b64(&salt)?;
        let params = scrypt::Params::new(memory, iterations, parallelism, key_length)?;
        let password_hash =
            MScrypt.hash_password_customized(input.as_bytes(), None, None, params, &salt)?;
        println!("{}", password_hash);

        Ok(password_hash.to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct Params {
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

create_me_daddy!();

pub struct Scrypt;
