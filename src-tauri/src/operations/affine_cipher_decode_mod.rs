use crate::{
    create_info_struct, run_op,
    traits::{CharTrait, IntegerTrait},
    utils::{
        get_alphabet, get_char_by_index, get_index_by_char, mod_inv, modulus, validate_lang,
        SupportedLanguages,
    },
    Operation, DOCS_URL,
};
use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

run_op!(run_affinecipherdecode, AffineCipherDecode);

impl Operation<'_, DeserializeMeDaddy> for AffineCipherDecode {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;

        if !validate_lang(input, &request.lang) {
            bail!("Wrong language.");
        };

        let (a, b) = (request.a as i16, request.b as i16);

        let (alp_lower, alp_upper, _, _, alp_length, _) = get_alphabet(&request.lang);
        if a.gcd(&(alp_length as i16)) != 1 {
            return Err(anyhow!(
                "The value of `a` must be coprime to alphabet length({}).",
                alp_length
            ));
        }

        let mut output = String::with_capacity(alp_length as usize);

        for c in input.chars() {
            if !c.is_alphabetic() {
                output.push(c);
                continue;
            }

            let y = match c.is_lowercase() {
                true => get_index_by_char(alp_lower, c),
                false => get_index_by_char(alp_upper, c),
            } as i16;

            let inv_a = mod_inv(a, alp_length as i16);

            let x = modulus(inv_a * (y - b), alp_length as i16);

            output.push(match c.is_lowercase() {
                true => get_char_by_index(alp_lower, x),
                false => get_char_by_index(alp_upper, x).to_upper_case(),
            });
        }

        Ok(output)
    }
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    lang: SupportedLanguages,
    a: u8,
    b: u8,
}

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
/// POST /api/AffineCipherDecode
///
/// {
///     "input": "Cnwwl, Zlawi!",
///     "lang": "en",
///     "params": {
///         "a": 5,
///         "b": 19
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "Hello, World!"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/AffineCipherDecode
///
/// {
///     "input": "Мскньы, Юкс!",
///     "params": {
///         "a": 5,
///         "b" 3,
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "Привет, Мир!"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/AffineCipherDecode
///
/// {
///     "input": "Cnwwl, Zlawi!",
///     "lang": "en",
///     "params": {
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
pub struct AffineCipherDecode;

const NAME: &str = "AffineCipherDecode";
const DESCRIPTION_EN: &str = "The Affine cipher is a type of monoalphabetic substitution cipher. To decrypt, each letter in an alphabet is mapped to its numeric equivalent, decrypted by a mathematical function, and converted back to a letter.";
const DESCRIPTION_RU: &str = "Аффинный шифр — это тип моноалфавитного шифра замены. Чтобы расшифровать, каждая буква в алфавите сопоставляется с ее числовым эквивалентом, расшифровывается с помощью математической функции и преобразуется обратно в букву.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Affine_cipher");

create_info_struct!(
    AffineCipherDecodeInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
