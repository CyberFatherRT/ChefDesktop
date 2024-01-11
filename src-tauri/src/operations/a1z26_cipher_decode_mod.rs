use crate::{create_info_struct, create_me_daddy, utils::char_repr, Operation, DOCS_URL, run_operations};
use serde::{Deserialize, Serialize};


impl Operation<'_, DeserializeMeDaddy, String> for A1Z26CipherDecode {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (input, delimiter) = (request.input, format!("{:?}", request.params.delimiter));

        let delimiter = char_repr(&delimiter);
        let cipher_text = input.split(delimiter);

        let mut plain_text = String::new();

        for c in cipher_text {
            let c = match c.parse::<u8>() {
                Ok(c) => c,
                Err(_) => continue,
            };
            if !(1..=26).contains(&c) {
                return Err("All numbers must be between 1 and 26.".to_string());
            }
            plain_text.push((c + 96) as char);
        }
        Ok(plain_text)
    }
}

#[derive(Deserialize)]
struct Params {
    delimiter: Delimiters,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Delimiters {
    Space,
    Comma,
    SemiColon,
    Colon,
    LineFeed,
    CRLF,
}

create_me_daddy!();

/// A1Z26 is a simple substitution cipher where each letter is replaced by its serial number in the alphabet.
/// <br/><br/>
/// # How to use
/// \
/// Send POST requests to /api/A1Z26CipherDecode with your data using json payload with this structure
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
/// POST /api/A1Z26CipherDecode
///
/// {
///     "input": "8 5 12 12 15",
///     "params": {
///         "delimiter": "Space"
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
///   "Err": "Invalid delimiter: `Unsupported delimiter`"
/// }
/// ```

pub struct A1Z26CipherDecode;

const NAME: &str = "A1Z26CipherDecode";
const DESCRIPTION_EN: &str =
    "Converts alphabet order numbers into their corresponding alphabet character.";
const DESCRIPTION_RU: &str =
    "Преобразует порядковые номера алфавита в соответствующие им символы алфавита.";

const INFO_URL: Option<&str> = None;

create_info_struct!(
    A1Z26CipherDecodeInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
