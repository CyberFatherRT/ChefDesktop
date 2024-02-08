use serde::{Deserialize, Serialize};

use crate::{
    create_info_struct, create_me_daddy, create_tauri_wrapper,
    libs::ciphers::affine_cipher_encode as ace, run_operations, utils::SupportedLanguages,
    Operation, DOCS_URL,
};
use anyhow::Result;

create_tauri_wrapper!(affine_cipher_encode, AffineCipherEncode);

impl Operation<'_, DeserializeMeDaddy> for AffineCipherEncode {
    fn do_black_magic(&self, request: &str) -> Result<String> {
        let request = self.validate(request)?;

        let (input, lang, a, b) = (
            request.input,
            request.params.lang,
            request.params.a as i16,
            request.params.b as i16,
        );

        ace(&input, lang, a, b)
    }
}

#[derive(Deserialize)]
struct Params {
    lang: SupportedLanguages,
    a: u8,
    b: u8,
}

create_me_daddy!();

/// The Affine cipher is a type of monoalphabetic substitution cipher. To decrypt, each letter in an alphabet is mapped to its numeric equivalent, decrypted by a mathematical function, and converted back to a letter.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Affine_cipher)
/// <br><br/>
/// # How to use
/// \
/// Send POST requests to /api/AffineCipherDecode with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "lang": SupportedLanguages
///         "a": u8,
///         "b": u8,
///     }
/// }
/// ```
/// #### where
///     - u8 is unsigned 8-bit integer (digit between 0 and 255)
///     - SupportedLanguages is enum of en, ru, ru_with_yo
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
/// POST /api/AffineCipherEncode
///
/// {
///     "input": "Hello, World!",
///     "params": {
///         "lang": "en",
///         "a": 5,
///         "b": 3
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "Mxggv, Jvkgs!"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/AffineCipherEncode
///
/// {
///     "input": "Привет, Мир!",
///     "params": {
///         "lang": "ru",
///         "a": 5,
///         "b" 3,
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "Мскньы, Юкс!"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/AffineCipherEncode
///
/// {
///     "input": "Hello, World!",
///     "params": {
///         "lang": "en",
///         "a": -5,
///         "b": 12735073052703957225979
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Invalid value: integer `-5`, expected u8."
/// }
/// ```
pub struct AffineCipherEncode;

const NAME: &str = "AffineCipherEncode";
const DESCRIPTION_EN: &str = "The Affine cipher is a type of monoalphabetic substitution cipher. To decrypt, each letter in an alphabet is mapped to its numeric equivalent, decrypted by a mathematical function, and converted back to a letter.";
const DESCRIPTION_RU: &str = "Аффинный шифр — это тип моноалфавитного шифра замены. Чтобы расшифровать, каждая буква в алфавите сопоставляется с ее числовым эквивалентом, расшифровывается с помощью математической функции и преобразуется обратно в букву.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Affine_cipher");

create_info_struct!(
    AffineCipherEncodeInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
