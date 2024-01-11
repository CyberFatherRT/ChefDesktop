use serde::{Deserialize, Serialize};

use crate::{
    create_info_struct, create_me_daddy, create_tauri_wrapper, libs::ciphers::affine_cipher_encode,
    run_operations, utils::SupportedLanguages, Operation, DOCS_URL,
};

create_tauri_wrapper!(atbash_cipher, AtbashCipher, String, String);

impl Operation<'_, DeserializeMeDaddy, String> for AtbashCipher {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;

        let (input, lang) = (request.input, request.params.lang);

        match lang {
            SupportedLanguages::EN => affine_cipher_encode(&input, lang, 25, 25),
            SupportedLanguages::RU => affine_cipher_encode(&input, lang, 31, 31),
            SupportedLanguages::RU_WITH_YO => affine_cipher_encode(&input, lang, 32, 32),
        }
    }
}

#[derive(Deserialize)]
struct Params {
    lang: SupportedLanguages,
}

create_me_daddy!();

/// Atbash is a mono-alphabetic substitution cipher originally used to encode the Hebrew alphabet. It has been modified here for use with the Latin alphabet and Cyrillic.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Atbash).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/AtbashCipherEncode with your data using json payload with this structure.
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "lang": SupportedLanguage,
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
/// POST /api/AtbashCipher
///
/// {
///     "input": "hello",
///     "params": {
///         "lang": "en"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "svool"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/AtbashCipher
///
/// {
///     "input": "Привет!",
///     "params": {
///         "lang": "ru",
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "Рпчэън!"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/AtbashCipher
///
/// {
///     "input": "no lang?",
///     "params": {
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `lang`"
/// }
/// ```
pub struct AtbashCipher;

const NAME: &str = "AtbashCipher";
const DESCRIPTION_EN: &str = "Atbash is a mono-alphabetic substitution cipher originally used to encode the Hebrew alphabet. It has been modified here for use with the Latin alphabet and Cyrillic.";
const DESCRIPTION_RU: &str = "Атбаш — это моноалфавитный шифр замены, изначально использовавшийся для кодирования еврейского алфавита. Здесь он был изменен для использования с латинским алфавитом и кириллицей";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Atbash");

create_info_struct!(
    AtbashCipherInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
