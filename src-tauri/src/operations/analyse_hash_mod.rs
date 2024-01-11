use crate::{create_info_struct, Operation, DOCS_URL};
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy, SerializeMeDaddy> for AnalyseHash {
    fn do_black_magic(&self, request: &str) -> Result<SerializeMeDaddy, String> {
        let request = self.validate(request)?;
        let input = request
            .input
            .chars()
            .map(|x| match x {
                '\t' | '\n' | ' ' => "".to_string(),
                _ => x.to_string(),
            })
            .collect::<String>();

        let hash_length = input.len();
        let byte_length: f64 = hash_length as f64 / 2.0;
        let bit_length: f64 = byte_length * 8.0;

        let possible_hash_functions = match bit_length as u16 {
            4 => vec!["Fletcher-4", "Luhn algorithm", "Verhoeff algorithm"],
            8 => vec!["Fletcher-8"],
            16 => vec!["BSD checksum", "CRC-16", "SYSV checksum", "Fletcher-16"],
            32 => vec!["CRC-32", "Fletcher-32", "Adler-32"],
            64 => vec!["CRC-64", "RIPEMD-64", "SipHash"],
            128 => vec![
                "MD5",
                "MD4",
                "MD2",
                "HAVAL-128",
                "RIPEMD-128",
                "Snefru",
                "Tiger-128",
            ],
            160 => vec![
                "SHA-1",
                "SHA-0",
                "FSB-160",
                "HAS-160",
                "HAVAL-160",
                "RIPEMD-160",
                "Tiger-160",
            ],
            192 => vec!["Tiger", "HAVAL-192"],
            224 => vec!["SHA-224", "SHA3-224", "ECOH-224", "FSB-224", "HAVAL-224"],
            256 => vec![
                "SHA-256",
                "SHA3-256",
                "BLAKE-256",
                "ECOH-256",
                "FSB-256",
                "GOST",
                "Grøstl-256",
                "HAVAL-256",
                "PANAMA",
                "RIPEMD-256",
                "Snefru",
            ],
            320 => vec!["RIPEMD-320"],
            384 => vec!["SHA-384", "SHA3-384", "ECOH-384", "FSB-384"],
            512 => vec![
                "SHA-512",
                "SHA3-512",
                "BLAKE-512",
                "ECOH-512",
                "FSB-512",
                "Grøstl-512",
                "JH",
                "MD6",
                "Spectral Hash",
                "SWIFFT",
                "Whirlpool",
            ],
            1024 => vec!["Fowler-Noll-Vo"],
            _ => {
                vec!["Unknown"]
            }
        };

        Ok(SerializeMeDaddy {
            hash_length,
            byte_length,
            bit_length,
            possible_hash_functions,
        })
    }
}

#[derive(Serialize)]
pub struct SerializeMeDaddy {
    hash_length: usize,
    byte_length: f64,
    bit_length: f64,
    possible_hash_functions: Vec<&'static str>,
}

#[derive(Deserialize)]
pub struct DeserializeMeDaddy {
    input: String,
}

/// This function tries to determine information about a given hash and suggests which algorithm may have been used to generate it based on its length.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Argon2)
/// <br/><br/>
///
/// # How to use
/// \
/// Send Post request to /api/AnalyseHash with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
/// }
/// ```
/// ### Server response have two possible formats
///
/// #### &nbsp;&nbsp;&nbsp;&nbsp; Ok variant
/// ``` json
/// {
///     "Ok": {
///         "hash_length": int,
///         "byte_length": float,
///         "bit_length": float,
///         "possible_hash_functions": [string]
///     }
/// }
/// #### where
///     - [string] is array of string
///
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
///     "input": "5eafacd17b1ee3cf06c4ef8e4e33ab31a86a62a1e9b8dda71c04c830",
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///     "Ok": {
///         "hash_length": 56,
///         "byte_length": 28.0,
///         "bit_length": 224.0,
///         "possible_hash_functions": [
///             "SHA-224",
///             "SHA3-224",
///             "ECOH-224",
///             "FSB-224",
///             "HAVAL-224"
///         ]
///     }
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/Argon2
///
/// {
///     "input": "867f9b0a82ec4923c5da5ff362fa67b745768294",
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///     "Ok": {
///         "hash_length": 40,
///         "byte_length": 20.0,
///         "bit_length": 160.0,
///         "possible_hash_functions": [
///             "SHA-1",
///             "SHA-0",
///             "FSB-160",
///             "HAS-160",
///             "HAVAL-160",
///             "RIPEMD-160",
///             "Tiger-160"
///         ]
///     }
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/Argon2
///
/// {
///     "no_input": "",
/// }
/// ```
/// ```http
/// {
///     "Err": "Missing field `input`."
/// }
/// ```
pub struct AnalyseHash;

const NAME: &str = "AnalyseHash";
const DESCRIPTION_EN: &str = "Tries to determine information about a given hash and suggests which algorithm may have been used to generate it based on its length.";
const DESCRIPTION_RU: &str = "Пытается определить информацию о заданном хэше и предлагает, какой алгоритм мог быть использован для его генерации, исходя из его длины.";
const INFO_URL: Option<&str> =
    Some("https://wikipedia.org/wiki/Comparison_of_cryptographic_hash_functions");

create_info_struct!(
    AnalyseHashInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
