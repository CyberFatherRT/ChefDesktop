use crate::{create_info_struct, Operation, DOCS_URL};

use anyhow::Result;
use base64::{alphabet, engine, Engine};
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy> for FromBase64 {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let mut input = input.to_string();
        let DeserializeMeDaddy {
            alphabet,
            remove_non_alphabetic_chars,
            strict_mode,
        } = request;

        let alphabet = alphabet.unwrap_or(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string(),
        );
        let alphabet = alphabet::Alphabet::new(&alphabet)?;

        if remove_non_alphabetic_chars {
            input = input
                .chars()
                .filter(|&x| alphabet.as_str().contains(x))
                .collect();
        }

        let config = engine::GeneralPurposeConfig::new()
            .with_decode_allow_trailing_bits(!strict_mode)
            .with_decode_padding_mode(engine::DecodePaddingMode::RequireCanonical);

        let engine = engine::GeneralPurpose::new(&alphabet, config);

        Ok(String::from_utf8(engine.decode(input)?)?)
    }
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    alphabet: Option<String>,
    remove_non_alphabetic_chars: bool,
    strict_mode: bool,
}

/// Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation decodes raw data into an ASCII Base64 string.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Base64).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/FromBase64 with your data using json payload with this structure.
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "alphabet": Option<string>,
///         "remove_non_alphabetic_chars": Option<bool>,
///         "strict_mode": Option<bool>
///     }
/// }
/// ```
/// #### where
///     - Option<string> is type that can be string or null.
///     - Option<bool> is type that can be bool or null.
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
/// POST /api/FromBase64
///
/// {
///     "input": "aGVsbG8=",
///     "params": {
///         "alphabet": "A-Za-z0-9+/=",
///         "remove_non_alphabetic_chars": false,
///         "strict_mode": false
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "hello"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/FromBase64
///
/// {
///     "input": ":&5L;&\",
///     "params": {
///         "alphabet": " -_",
///         "remove_non_alphabetic_chars": false,
///         "strict_mode": false
///     }
/// }
/// ``
/// ```
/// ```http
/// {
///   "Ok": "hello"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/FromBase64
/// content_type: application/json; charset=utf-8
///
/// {
///     "input": ":&5L;&\",
///     "params": {
///         "alphabet": " -_",
///         "remove_non_alphabetic_chars": false
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `strict_mode`."
/// }
/// ```
pub struct FromBase64;

const NAME: &str = "FromBase64";
const DESCRIPTION_EN: &str = "Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation decodes raw data into an ASCII Base64 string.";
const DESCRIPTION_RU: &str = "Base64 — это нотация для кодирования произвольных байтовых данных с использованием ограниченного набора символов, которые могут удобно использоваться людьми и обрабатываться компьютерами.<br><br>Эта операция декодирует необработанные данные в строку ASCII Base64.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Base64");

create_info_struct!(
    FromBase64Info,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
