use crate::{OutputFormat, create_tauri_wrapper};
use crate::{
    create_info_struct, create_me_daddy,
    libs::base64::to_base64,
    utils::{convert_to_byte_array, to_hex, SupportedFormats},
    Operation, DOCS_URL, run_operations
};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2s,
};
use serde::{Deserialize, Serialize};

create_tauri_wrapper!(blake2s, Blake2s, OutputFormat, String);

impl Operation<'_, DeserializeMeDaddy, OutputFormat> for Blake2s {
    fn do_black_magic(&self, request: &str) -> Result<OutputFormat, String> {
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
                None => {
                    return Err("Key format argument must be set.".to_string());
                }
                Some(key_format) => convert_to_byte_array(&key, &key_format)?,
            },
        };

        let mut hasher = VarBlake2s::new_keyed(
            &key,
            match size {
                SupportedBlake2sSize::Blake2s128 => 16,
                SupportedBlake2sSize::Blake2s160 => 20,
                SupportedBlake2sSize::Blake2s256 => 32,
                SupportedBlake2sSize::Blake2s384 => 48,
                SupportedBlake2sSize::Blake2s512 => 64,
            },
        );

        hasher.update(input.as_bytes());

        let res = hasher.finalize_boxed();

        Ok(match output_format {
            SupportedOutputFormat::Hex => OutputFormat::Hex(to_hex(&res)),
            SupportedOutputFormat::Base64 => OutputFormat::Base64(to_base64(&res, None)?),
            SupportedOutputFormat::Uint8Array => OutputFormat::Uint8Array(res.to_vec()),
        })
    }
}

#[derive(Deserialize)]
enum SupportedBlake2sSize {
    #[serde(rename = "128")]
    Blake2s128,
    #[serde(rename = "160")]
    Blake2s160,
    #[serde(rename = "256")]
    Blake2s256,
    #[serde(rename = "384")]
    Blake2s384,
    #[serde(rename = "512")]
    Blake2s512,
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
    size: SupportedBlake2sSize,
    output_format: SupportedOutputFormat,
}

create_me_daddy!();

/// BLAKE2s is a flavour of the BLAKE cryptographic hash function that is optimized for 8- to 32-bit platforms and produces digests of any size between 1 and 32 bytes.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE2).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/Blake2s with your data using json payload with this structure.
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "key": Option<String>,
///         "key_format": Option<SupportedFormats>,
///         "size": SupportedBlake2sSize,
///         "output_format": SupportedOutputFormat
///     }
/// }
/// ```
/// #### where
///     - Option<String> is optional argument with type of string
///     - Option<SupportedFormats> is optional enum of "binary", "utf8", "hex", "base64", "latin1"
///     - SupportedBlake2sSize is enum of "128", "160", "256", "384", "512"
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
/// POST /api/Blake2s
///
/// {
///     "input": "hello world",
///     "params": {
///         "size": "160",
///         "key": "c3VwZXIgc2VjcmV0IGtleQ=",
///         "key_format": "base64",
///         "output_format": "base64"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///     "Ok": {
///         "base64": "7Y12BJIsWVVt5Oj36MRs0OPzN4o="
///     }
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/Blake2s
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
///         "hex": "96d539653dbf841c384b53d5f04658e5"
///     }
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/Blake2s
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
pub struct Blake2s;

const NAME: &str = "Blake2s";
const DESCRIPTION_EN: &str = "Performs BLAKE2s hashing on the input.";
const DESCRIPTION_RU: &str = "Выполняет хеширование BLAKE2s на входных данных.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE2");

create_info_struct!(
    Blake2sInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
