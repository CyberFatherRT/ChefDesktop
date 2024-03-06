use crate::{
    create_info_struct,
    libs::bacon::{BaconCipher, SupportedBaconAlphabet, SupportedBaconTranslation},
    run_op,
    utils::SupportedLanguages,
    Operation, DOCS_URL,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::mem::swap;

run_op!(run_baconcipherencode, BaconCipherEncode);

impl Operation<'_, DeserializeMeDaddy> for BaconCipherEncode {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let DeserializeMeDaddy {
            bacon_alphabet,
            translation,
            keep_extra_character,
            invert_translation,
            lang,
        } = request;

        let (mut a, mut b) = match translation {
            SupportedBaconTranslation::ZeroOne => ('0', '1'),
            SupportedBaconTranslation::AB => ('A', 'B'),
        };

        if invert_translation {
            swap(&mut a, &mut b);
        }

        let cipher = BaconCipher::new(a, b, translation, bacon_alphabet, lang);

        let output = cipher.encode(&input);

        let output = if keep_extra_character {
            output.join("")
        } else {
            output
                .iter()
                .filter(|x| x.len() != 1)
                .cloned()
                .collect::<Vec<String>>()
                .join(" ")
        }
        .to_uppercase();

        Ok(output)
    }
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    bacon_alphabet: SupportedBaconAlphabet,
    translation: SupportedBaconTranslation,
    keep_extra_character: bool,
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
/// Send POST requests to /api/BaconCipherEncode with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         bacon_alphabet: SupportedBaconAlphabet,
///         translation: SupportedBaconTranslation,
///         keep_extra_character: bool,
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
/// POST /api/BaconCipherEncode
///
/// {
///     "input": "string",
///     "params": {
///         bacon_alphabet: "Standard",
///         translation: "0/1",
///         keep_extra_character: false,
///         invert_translation: false,
///         lang: "en",
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "10001 10010 10000 01000 01100 00110"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/BaconCipherEncode
///
/// {
///     "input": "Hello, World!",
///     "params": {
///         bacon_alphabet: "Complete",
///         translation: "A/B",
///         keep_extra_character: true,
///         invert_translation: true,
///         lang: "en",
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "BBAAABBABBBABAABABAABAAAB, ABAABBAAABABBBABABAABBBAA!"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/BaconCipherEncode
///
/// {
///     "input": "missing `lang`",
///     "params": {
///         bacon_alphabet: "Standard",
///         translation: "0/1",
///         keep_extra_character: true,
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
pub struct BaconCipherEncode;

const NAME: &str = "Bacon Cipher Encode";
const DESCRIPTION_EN: &str =
    "Bacon's cipher or the Baconian cipher is a method of steganography devised by Francis Bacon in 1605. A message is concealed in the presentation of text, rather than its content.";
const DESCRIPTION_RU: &str = "Шифр Бэкона — это метод стеганографии, разработанный Фрэнсисом Бэконом в 1605 году. Сообщение скрыто в представлении текста, а не в его содержании.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Bacon%27s_cipher");

create_info_struct!(
    BaconCipherEncodeInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
