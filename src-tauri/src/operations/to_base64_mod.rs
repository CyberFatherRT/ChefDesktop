use anyhow::Result;
use base64::{alphabet, engine, Engine};
use serde::{Deserialize, Serialize};

use crate::{
    create_info_struct, create_me_daddy, create_tauri_wrapper, run_operations, Operation, DOCS_URL,
};

create_tauri_wrapper!(to_base64, ToBase64);

impl Operation<'_, DeserializeMeDaddy> for ToBase64 {
    fn do_black_magic(&self, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let (input, alphabet) = (request.input, request.params.alphabet);

        let alphabet = alphabet.unwrap_or(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string(),
        );
        let alphabet = alphabet::Alphabet::new(&alphabet)?;

        let config = engine::GeneralPurposeConfig::new()
            .with_decode_padding_mode(engine::DecodePaddingMode::RequireCanonical);
        let engine = engine::GeneralPurpose::new(&alphabet, config);

        Ok(engine.encode(input))
    }
}

#[derive(Deserialize)]
struct Params {
    alphabet: Option<String>,
}

create_me_daddy!();

/// Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation decodes raw data into an ASCII Base64 string.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Base64).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/ToBase64 with your data using json payload with this structure.
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "alphabet": Option<string>,
///     }
/// }
/// ```
/// #### where
///     - Option<string> is type that can be string or null.
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
/// POST /api/ToBase64
///
/// {
///     "input": "hello",
///     "params": {
///         "alphabet": "A-Za-z0-9+/=",
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": aGVsbG8="
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/ToBase64
///
/// {
///     "input": "hello",
///     "params": {
///         "alphabet": " -_",
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": ":&5L;&\"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/ToBase64
///
/// {
///     "params": {
///         "alphabet": "t/RPURZbnkPVD="
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `input`"
/// }
/// ```
pub struct ToBase64;

const NAME: &str = "ToBase64";
const DESCRIPTION_EN: &str = "Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation decodes raw data into an ASCII Base64 string.";
const DESCRIPTION_RU: &str = "Base64 — это нотация для кодирования произвольных байтовых данных с использованием ограниченного набора символов, которые могут удобно использоваться людьми и обрабатываться компьютерами.<br><br>Эта операция декодирует необработанные данные в строку ASCII Base64.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Base64");

create_info_struct!(
    ToBase64Info,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
