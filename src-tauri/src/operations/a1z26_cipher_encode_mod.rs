use crate::{
    create_info_struct, create_me_daddy, create_tauri_wrapper, operations::Delimiters,
    run_operations, utils::char_repr, Operation, DOCS_URL,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

create_tauri_wrapper!(a1z26_cipher_encode, A1Z26CipherEncode);

impl Operation<'_, DeserializeMeDaddy> for A1Z26CipherEncode {
    fn do_black_magic(&self, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let (input, delimiter) = (request.input, format!("{:?}", request.params.delimiter));

        let mut result = String::new();
        let delimiter = char_repr(&delimiter);

        for character in input.chars() {
            result.push_str(&match character {
                'a'..='z' => format!("{}{}", character as u8 - 96, delimiter),
                'A'..='Z' => format!("{}{}", character as u8 - 64, delimiter),
                _ => "".to_string(),
            });
        }
        if result.is_empty() {
            return Ok(String::new());
        }
        Ok(result[..result.len() - 1].to_string())
    }
}

#[derive(Deserialize)]
struct Params {
    delimiter: Delimiters,
}

create_me_daddy!();

/// A1Z26 is a simple substitution cipher where each letter is replaced by its serial number in the alphabet.
/// <br/><br/>
/// # How to use
/// \
/// Send POST requests to /api/A1Z26CipherEncode with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "delimiter": string
///     }
/// }
/// ```
/// #### where
///     - delimiter is one of "Space", "Comma", "SemiColon", "Colon", "LineFeed", "CRLF"
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
/// <br><br/>
/// ## №1
/// ``` http
/// POST /api/A1Z26CipherEncode
///
/// {
///     "input": "hello",
///     "params": {
///         "delimiter": "Space"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "8 5 12 12 15"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/A1Z26CipherDecode
///
/// {
///     "input": "18;9;3;11;18;15;12;12",
///     "params": {
///         "delimiter": "Semi-colon"
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "rickroll"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/A1Z26CipherDecode
///
/// {
///     "input": "4 1 21 15 3",
///     "params": {
///         "delimiter": "Unsupported delimiter"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Invalid delimiter: `Unsupported delimiter`."
/// }
/// ```

pub struct A1Z26CipherEncode;

const NAME: &str = "A1Z26CipherEncode";
const DESCRIPTION_EN: &str =
    "Converts alphabet characters into their corresponding alphabet order number.";
const DESCRIPTION_RU: &str =
    "Преобразует символы алфавита в соответствующие им порядковые номера алфавита.";

const INFO_URL: Option<&str> = None;

create_info_struct!(
    A1Z26CipherEncodeInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
