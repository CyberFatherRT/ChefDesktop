use serde::{Deserialize, Serialize};

use crate::{
    create_info_struct, create_me_daddy,
    libs::vigenere_trait::VigenereCipher,
    utils::{add, SupportedLanguages},
    Operation, DOCS_URL, create_tauri_wrapper, run_operations
};

create_tauri_wrapper!(vigenere_cipher_encode, VigenereCipherEncode, String, String);

impl VigenereCipher for VigenereCipherEncode {}

impl Operation<'_, DeserializeMeDaddy, String> for VigenereCipherEncode {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (input, lang, key) = (request.input, request.params.lang, request.params.key);
        <Self as VigenereCipher>::cipher(lang, &key, &input, add)
    }
}

#[derive(Deserialize)]
struct Params {
    lang: SupportedLanguages,
    key: String,
}
create_me_daddy!();

/// The Vigenere cipher is a method of encrypting alphabetic text by using a series of different Caesar common based on the letters of a keyword. It is a simple form of polyalphabetic substitution.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Vigenère_cipher).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/VigenereCipherEncode with your data using json payload with this structure.
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "lang": SupportedLanguages,
///         "key": String
///     }
/// }
/// ```
/// #### where
///     - SupportedLanguages is enum of "en", "ru", "ru_with_yo".
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
/// POST /api/VigenereCipherEncode
///
/// {
///     "input": "Hello, World!",
///     "params": {
///         "lang": "en",
///         "key": "key"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "Rijvs, Uyvjn!"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/VigenereCipherEncode
///
/// {
///     "input": "Привет, Мир!",
///     "params": {
///         "lang": "ru",
///         "key": "ключ"
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "Ееклыз, Осж!"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/VigenereCipherEncode
///
/// {
///     "input": "wrong language",
///     "params": {
///         "lang": "else",
///         "key": "lang"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Unknown variant `else`, expected one of `En`, `Ru`, `RuAlpWithYo`."
/// }
/// ```
pub struct VigenereCipherEncode;

const NAME: &str = "VigenereEncode";
const DESCRIPTION_EN: &str = "The Vigenere cipher is a method of encrypting alphabetic text by using a series of different Caesar common based on the letters of a keyword. It is a simple form of polyalphabetic substitution.";
const DESCRIPTION_RU: &str = "Шифр Виженера — это метод шифрования алфавитного текста с использованием ряда различных общих символов Цезаря, основанных на буквах ключевого слова. Это простая форма полиалфавитной замены.";
const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Vigenère_cipher");

create_info_struct!(
    VigenereCipherEncodeInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
