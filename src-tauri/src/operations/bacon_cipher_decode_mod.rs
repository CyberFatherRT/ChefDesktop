use crate::{
    create_info_struct, create_me_daddy, create_tauri_wrapper,
    libs::bacon::{BaconCipher, SupportedBaconAlphabet, SupportedBaconTranslation},
    run_operations,
    utils::SupportedLanguages,
    Operation, DOCS_URL,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::mem::swap;

create_tauri_wrapper!(bacon_cipher_decode, BaconCipherDecode);

impl Operation<'_, DeserializeMeDaddy> for BaconCipherDecode {
    fn do_black_magic(&self, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let (
            input,
            Params {
                bacon_alphabet,
                translation,
                invert_translation,
                lang,
            },
        ) = (request.input, request.params);

        let (mut a, mut b) = match translation {
            SupportedBaconTranslation::ZeroOne => ('0', '1'),
            SupportedBaconTranslation::AB => ('A', 'B'),
        };

        if invert_translation {
            swap(&mut a, &mut b);
        }

        let cipher = BaconCipher::new(a, b, translation, bacon_alphabet, lang);

        let output = cipher.decode(&input).join("");

        Ok(output)
    }
}

create_me_daddy!();

#[derive(Deserialize)]
struct Params {
    bacon_alphabet: SupportedBaconAlphabet,
    translation: SupportedBaconTranslation,
    invert_translation: bool,
    lang: SupportedLanguages,
}

/// Bacon's cipher or the Baconian cipher is a method of steganography devised by Francis Bacon in 1605. A message is concealed in the presentation of text, rather than its content.
/// <br><br/>
/// For more information about cipher/hash_function go [here](https://wikipedia.org/wiki/Bacon%27s_cipher)\
/// For more information about this function go [here](DOCS_URL)
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/BaconCipherDecode with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         bacon_alphabet: SupportedBaconAlphabet,
///         translation: SupportedBaconTranslation,
///         invert_translation: bool,
///         lang: SupportedLanguages,
///     }
/// }
/// ```
/// #### where
///     - SupportedBaconAlphabet is enum of "Standard", "Complete"
///     - SupportedBaconTranslation is enum of "0/1", "A/B"
///     - SupportedLanguages is enum of "en", "ru", "ru_with_yo"
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
/// POST /api/BaconCipherDecode
///
/// {
///     "input": "10001 10010 10000 01000 01100 00110",
///     "params": {
///         bacon_alphabet: "Standard",
///         translation: "0/1",
///         invert_translation: false,
///         lang: "en",
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "string"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/BaconCipherDecode
///
/// {
///     "input": "BBAAA BBABB BABAA BABAA BAAAB ABAAB BAAAB ABBBA BABAA BBBAA",
///     "params": {
///         bacon_alphabet: "Complete",
///         translation: "A/B",
///         invert_translation: true,
///         lang: "en",
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "helloworld"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/BaconCipherDecode
///
/// {
///     "input": "missing `lang`",
///     "params": {
///         bacon_alphabet: "Standard",
///         translation: "0/1",
///         invert_translation: false,
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `lang`."
/// }
/// ```
pub struct BaconCipherDecode;

const NAME: &str = "Bacon Cipher Decode";
const DESCRIPTION_EN: &str =
    "Bacon's cipher or the Baconian cipher is a method of steganography devised by Francis Bacon in 1605. A message is concealed in the presentation of text, rather than its content.";
const DESCRIPTION_RU: &str = "Шифр Бэкона — это метод стеганографии, разработанный Фрэнсисом Бэконом в 1605 году. Сообщение скрыто в представлении текста, а не в его содержании.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Bacon%27s_cipher");

create_info_struct!(
    BaconCipherDecodeInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
