use anyhow::Result;

use hmac::{Hmac, Mac};
use md2::*;
use md4::*;
use md5::*;
use ripemd::*;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::*;
use whirlpool::*;

use crate::{
    create_info_struct,
    libs::base64::to_base64,
    run_op,
    utils::{convert_to_byte_array, to_hex, SupportedFormats},
    Operation, DOCS_URL,
};

run_op!(run_hmac, HMAC);

impl Operation<'_, DeserializeMeDaddy> for HMAC {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;

        let (key, key_format, hash_function, output_format) = (
            request.key,
            request.key_format,
            request.hash_function,
            request.output_format,
        );

        let key = convert_to_byte_array(&key, &key_format)?;

        let res = match hash_function {
            SupportedHashFunctions::MD2 => {
                let mut hasher = HmacMD2::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::MD4 => {
                let mut hasher = HmacMD4::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::MD5 => {
                let mut hasher = HmacMD5::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::SHA1 => {
                let mut hasher = HmacSha1::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::SHA224 => {
                let mut hasher = HmacSha224::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::SHA256 => {
                let mut hasher = HmacSha256::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::SHA384 => {
                let mut hasher = HmacSha384::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::SHA512 => {
                let mut hasher = HmacSha512::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::SHA512_224 => {
                let mut hasher = HmacSha512_224::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::SHA512_256 => {
                let mut hasher = HmacSha512_256::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::Ripemd128 => {
                let mut hasher = HmacRipemd128::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::Ripemd160 => {
                let mut hasher = HmacRipemd160::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::Ripemd256 => {
                let mut hasher = HmacRipemd256::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::Ripemd320 => {
                let mut hasher = HmacRipemd320::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
            SupportedHashFunctions::WhirlPool => {
                let mut hasher = HmacWhirlPool::new_from_slice(&key)?;
                hasher.update(input.as_bytes());
                hasher.finalize().into_bytes().to_vec()
            }
        };

        Ok(match output_format {
            SupportedOutputFormat::Hex => to_hex(&res),
            SupportedOutputFormat::Base64 => to_base64(&res),
            SupportedOutputFormat::Raw => String::from_utf8_lossy(&res).to_string(),
        })
    }
}

type HmacMD2 = Hmac<Md2>;
type HmacMD4 = Hmac<Md4>;
type HmacMD5 = Hmac<Md5>;
type HmacSha1 = Hmac<Sha1>;
type HmacSha224 = Hmac<Sha224>;
type HmacSha256 = Hmac<Sha256>;
type HmacSha384 = Hmac<Sha384>;
type HmacSha512 = Hmac<Sha512>;
type HmacSha512_224 = Hmac<Sha512_224>;
type HmacSha512_256 = Hmac<Sha512_256>;
type HmacRipemd128 = Hmac<Ripemd128>;
type HmacRipemd160 = Hmac<Ripemd160>;
type HmacRipemd256 = Hmac<Ripemd256>;
type HmacRipemd320 = Hmac<Ripemd320>;
type HmacWhirlPool = Hmac<Whirlpool>;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum SupportedHashFunctions {
    MD2,
    MD4,
    MD5,
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    SHA512_224,
    SHA512_256,
    Ripemd128,
    Ripemd160,
    Ripemd256,
    Ripemd320,
    WhirlPool,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum SupportedOutputFormat {
    Hex,
    Base64,
    Raw,
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    key: String,
    key_format: SupportedFormats,
    hash_function: SupportedHashFunctions,
    output_format: SupportedOutputFormat,
}

/// Keyed-Hash Message Authentication Codes (HMAC) are a mechanism for message authentication using cryptographic hash functions.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/HMAC)
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/HMAC with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "key": string,
///         "key_format": SupportedFormats,
///         "hash_function": SupportedHashFunctions,
///         "output_format": SupportedOutputFormat
///     }
/// }
/// ```
/// #### where
///     - SupportedFormat is enum of "binary", "utf8", "hex", "base64", "latin1"
///     - SupportedHashFunctions is enum of "md2", "md4", "md5", "sha1", "sha224", "sha256", "sha384", "sha512", "sha512_224", "sha512_256", "ripemd128", "ripemd160", "ripemd256", "ripemd320", "whirlpool"
///     - SupportedOutputFormat is enum of "hex", "base64", "raw"
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
/// POST /api/HMAC
///
/// {
///     "input": "hello",
///     "params": {
///         "key": "key",
///         "key_format": "utf8",
///         "hash_function": "ripemd128",
///         "output_format": "hex"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": {
///     "hex": "8ff23128d3b4d93a58d740fe66dc86d4"
///   }
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/HMAC
///
/// {
///     "input": "deadbeef",
///     "params": {
///         "key": "ZGVhZGJlZWY=",
///         "key_format": "base64",
///         "hash_function": "sha256",
///         "output_format": "base64"
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": {
///     "base64": "fAzUwzhmrZvCxmTkLc0RbvoNTfORtwk9hDkv2NTvvrU"
///   }
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/HMAC
///
/// {
///     "input": "error",
///     "params": {
///         "key": "no function",
///         "key_format": "latin1",
///         "output_format": "uint8array"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `hash_function`"
/// }
/// ```
pub struct HMAC;

const NAME: &str = "HMAC";
const DESCRIPTION_EN: &str = "Keyed-Hash Message Authentication Codes (HMAC) are a mechanism for message authentication using cryptographic hash functions.";
const DESCRIPTION_RU: &str = "Keyed-Hash Message Authentication Codes (HMAC) — это механизм аутентификации сообщений с использованием криптографических хеш-функций.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/HMAC");

create_info_struct!(
    HmacInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
