use crate::{
    create_info_struct, libs::base64::from_base64, run_op, utils::to_hex, Operation, DOCS_URL,
};
use anyhow::{Error, Result};
use argon2::{Config, Variant, Version};
use serde::{Deserialize, Serialize};

run_op!(run_argon2, Argon2);

impl Operation<'_, DeserializeMeDaddy> for Argon2 {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;

        let (salt, variant, mem_cost, time_cost, lanes, hash_length) = (
            request.salt,
            request.argon2_type,
            request.memory,
            request.iterations,
            request.parallelism,
            request.hash_length,
        );

        let config = Config {
            variant,
            version: Version::Version13,
            mem_cost,
            time_cost,
            lanes,
            secret: &[],
            ad: &[],
            hash_length,
        };

        let hash = argon2::hash_encoded(input.as_bytes(), salt.as_bytes(), &config)?;

        let output = match request.output_format {
            OutputFormat::Encoded => hash,
            format @ (OutputFormat::Hex | OutputFormat::Raw) => {
                let raw_hash = hash
                    .split('$')
                    .nth(5)
                    .ok_or(Error::msg("Not valid argon2 hash"))?;

                let data = from_base64(raw_hash)?;

                match format {
                    OutputFormat::Hex => to_hex(&data),
                    OutputFormat::Raw => String::from_utf8(data)?,
                    _ => unreachable!(),
                }
            }
        };

        Ok(output)
    }
}

#[derive(Deserialize)]
#[serde(remote = "Variant")]
enum MyVariant {
    Argon2d = 0,
    Argon2i = 1,
    Argon2id = 2,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum OutputFormat {
    Encoded,
    Hex,
    Raw,
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    salt: String,
    iterations: u32,
    memory: u32,
    parallelism: u32,
    hash_length: u32,
    #[serde(with = "MyVariant")]
    argon2_type: Variant,
    output_format: OutputFormat,
}

/// Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition in July 2015. It was designed by Alex Biryukov, Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Argon2)
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/Argon2 with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "salt": string,
///         "iterations": u32,
///         "parallelism": u32,
///         "hash_length": u32,
///         "argon2_type": Argon2Type,
///         "output_format": OutputFormat,
///         "memory": u32
///     }
/// }
/// ```
/// #### where
///     - u32 is unsigned 32-bit integer
///     - SaltFormat is enum of "utf8", "hex", "base64", "latin1"
///     - Argon2Type is enum of "Argon2i", "Argon2d", "Argon2id"
///     - OutputFormat is enum of "encoded", "hex", "raw"
/// <br/><br/>
///
/// ### Server response have two possible formats
///
/// #### &nbsp;&nbsp;&nbsp;&nbsp; Ok variant
/// ``` json
/// { "Ok": `some answer` }
/// ```
/// #### &nbsp;&nbsp;&nbsp;&nbsp; Error variant
/// ``` json
/// { "Err": `error message` }
/// ```
/// # Examples
/// ## №1
/// ``` http
/// POST /api/Argon2
///
/// {
///     "input": "hello",
///     "params": {
///         "salt": "somesalt",
///         "iterations": 3,
///         "parallelism": 1,
///         "hash_length": 32,
///         "argon2_type": "Argon2i",
///         "output_format": "encoded",
///         "memory": 4096
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "$argon2i$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$WVDOfucSPAey3UEzzqLtBwRbGS83pTyIPLXgjhKfgrY"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/Argon2
///
/// {
///     "input": "Привет, Мир!",
///     "params": {
///         "salt": "123456789",
///         "iterations": 6,
///         "parallelism": 1,
///         "hash_length": 34,
///         "argon2_type": "Argon2id",
///         "output_format": "hex",
///         "memory": 8096
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "hex"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/Argon2
///
/// {
///     "input": "error",
///     "params": {
///         "salt": "missing iterations parameter",
///         "parallelism": 1,
///         "hash_length": 34,
///         "argon2_type": "Argon2id",
///         "output_format": "hex",
///         "memory": 8096
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `iterations`"
/// }
/// ```
pub struct Argon2;

const NAME: &str = "Argon2";
const DESCRIPTION_EN: &str = "Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition in July 2015. It was designed by Alex Biryukov, Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg.<br><br>Enter the password in the input to generate its hash.";
const DESCRIPTION_RU: &str = "Argon2 – это функция получения ключа, которая была выбрана победителем конкурса хеширования паролей в июле 2015 года. Она была разработана Алексом Бирюковым, Даниэлем Дину и Дмитрием Ховратовичем из Люксембургского университета.<br><br>Введите пароль в ввод для генерации его хэша.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Argon2");

create_info_struct!(
    Argon2Info,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
