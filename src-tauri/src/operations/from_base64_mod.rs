use serde::{Deserialize, Serialize};

use crate::{
    create_info_struct, create_me_daddy,
    libs::base64::from_base64,
    utils::{DataRepresentation, DataRepresentationInput},
    Operation, DOCS_URL,
};

impl Operation<'_, DeserializeMeDaddy, String> for FromBase64 {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;

        let (input, alphabet, remove_non_alphabetic_chars, strict_mode) = (
            request.input,
            request.params.alphabet,
            request
                .params
                .remove_non_alphabetic_chars
                .unwrap_or_default(),
            request.params.strict_mode.unwrap_or_default(),
        );

        let alphabet = alphabet.unwrap_or_default();

        match from_base64(
            input,
            &alphabet,
            DataRepresentationInput::String,
            remove_non_alphabetic_chars,
            strict_mode,
        ) {
            Ok(output) => {
                let DataRepresentation::String(output) = output else {
                    unreachable!()
                };
                Ok(output.trim_end_matches('\0').to_string())
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Deserialize)]
struct Params {
    alphabet: Option<String>,
    remove_non_alphabetic_chars: Option<bool>,
    strict_mode: Option<bool>,
}

create_me_daddy!();

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
