use crate::{
    create_info_struct, create_me_daddy, create_tauri_wrapper,
    libs::base64::bytes_to_base64,
    run_operations,
    utils::{convert_to_byte_array, to_hex, SupportedFormats},
    Operation, DOCS_URL,
};
use anyhow::{bail, Result};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use serde::{Deserialize, Serialize};

create_tauri_wrapper!(blake2b, Blake2b);

impl Operation<'_, DeserializeMeDaddy> for Blake2b {
    fn do_black_magic(&self, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let (input, size, key, key_format, output_format) = (
            request.input,
            request.params.size,
            request.params.key,
            request.params.key_format,
            request.params.output_format,
        );

        let key = match key {
            None => Vec::new(),
            Some(key) => match key_format {
                None => bail!("Key format argument must be set."),
                Some(key_format) => convert_to_byte_array(&key, &key_format)?,
            },
        };

        let mut hasher = VarBlake2b::new_keyed(
            &key,
            match size {
                SupportedBlake2bSize::Blake2b128 => 16,
                SupportedBlake2bSize::Blake2b160 => 20,
                SupportedBlake2bSize::Blake2b256 => 32,
                SupportedBlake2bSize::Blake2b384 => 48,
                SupportedBlake2bSize::Blake2b512 => 64,
            },
        );

        hasher.update(input.as_bytes());

        let res = hasher.finalize_boxed();

        Ok(match output_format {
            SupportedOutputFormat::Hex => to_hex(&res),
            SupportedOutputFormat::Base64 => bytes_to_base64(&res),
            SupportedOutputFormat::Uint8Array => String::from_utf8_lossy(&res).to_string(),
        })
    }
}

#[derive(Deserialize)]
enum SupportedBlake2bSize {
    #[serde(rename = "128")]
    Blake2b128,
    #[serde(rename = "160")]
    Blake2b160,
    #[serde(rename = "256")]
    Blake2b256,
    #[serde(rename = "384")]
    Blake2b384,
    #[serde(rename = "512")]
    Blake2b512,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum SupportedOutputFormat {
    Hex,
    Base64,
    Uint8Array,
}

#[derive(Deserialize)]
struct Params {
    key: Option<String>,
    key_format: Option<SupportedFormats>,
    size: SupportedBlake2bSize,
    output_format: SupportedOutputFormat,
}

create_me_daddy!();

/// BLAKE2b is a flavour of the BLAKE cryptographic hash function that is optimized for 64-bit platforms and produces digests of any size between 1 and 64 bytes.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE2b_algorithm).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/Blake2b with your data using json payload with this structure.
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "key": Option<String>,
///         "key_format": Option<SupportedFormats>,
///         "size": SupportedBlake2bSize,
///         "output_format": SupportedOutputFormat
///     }
/// }
/// ```
/// #### where
///     - Option<String> is optional argument with type of string
///     - Option<SupportedFormats> is optional enum of "binary", "utf8", "hex", "base64", "latin1"
///     - SupportedBlake2bSize is enum of "128", "160", "256", "384", "512"
///     - SupportedOutputFormat is enum of "hex", "base64", "uint8array"
/// <br/><br/>
///
/// ### Server response have two possible formats
///
/// #### &nbsp;&nbsp;&nbsp;&nbsp; Ok variant
/// ``` json
/// {
///   "Ok": {
///     "hex|base64|uint8array": "string|uint8array"
///   }
/// }
/// ```
/// #### &nbsp;&nbsp;&nbsp;&nbsp; Error variant
/// ``` json
/// { "Err": `error message` }
/// ```
/// # Examples
/// ## №1
/// ``` http
/// POST /api/Blake2b
///
/// {
///     "input": "hello world",
///     "params": {
///         "size": "160",
///         "key": "super secret key",
///         "output_format": "base64"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///     "Ok": {
///         "base64": "ktyh3oJo3OG/51S6SrnE6lzvSuk="
///     }
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/Blake2b
///
/// {
///     "input": "hello",
///     "params": {
///         "size": "128",
///         "output_format": "hex"
///     }
/// }
/// ```
/// ```http
/// {
///     "Ok": {
///         "hex": "46fb7408d4f285228f4af516ea25851b"
///     }
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/Blake2b
///
/// {
///     "input": "hello",
///     "params": {
///         "size": "128",
///         "key": "key format not set"
///         "output_format": "hex"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Key format argument must be set."
/// }
/// ```
pub struct Blake2b;

const NAME: &str = "Blake2b";
const DESCRIPTION_EN: &str = "Performs BLAKE2b hashing on the input.";
const DESCRIPTION_RU: &str = "Выполняет хеширование BLAKE2b на входных данных.";

const INFO_URL: Option<&str> =
    Some("https://wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE2b_algorithm");

create_info_struct!(
    Blake2bInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
