use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    create_info_struct, create_me_daddy,
    libs::ciphers::gen_polybius_square,
    regex_check,
    traits::CharTrait,
    utils::{get_alphabet, SupportedLanguages},
    Operation, DOCS_URL,
};

impl Operation<'_, DeserializeMeDaddy, String> for BifidCipherEncode {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (input, lang, keyword) = (request.input, request.params.lang, request.params.keyword);
        let keyword_str = match lang {
            SupportedLanguages::EN => keyword.to_uppercase().replace('J', "I"),
            SupportedLanguages::RU | SupportedLanguages::RU_WITH_YO => keyword.to_uppercase(),
        };
        let keyword: String = keyword_str.chars().dedup().collect();
        let (_, _, _, _, _, reg) = get_alphabet(&lang);
        if !regex_check!(reg => &keyword_str) && keyword.is_empty() {
            return Err("The key must consist only of your alphabets characters".to_string());
        }

        let (input, size) = match lang {
            SupportedLanguages::EN => (input.replace('J', "I").replace('j', "i"), 5),
            SupportedLanguages::RU | SupportedLanguages::RU_WITH_YO => (input, 6),
        };

        let mut x_cord: Vec<usize> = Vec::new();
        let mut y_cord: Vec<usize> = Vec::new();
        let mut other: HashMap<usize, char> = HashMap::new();
        let mut case: Vec<bool> = Vec::new();
        let mut output = String::new();

        let polybius = gen_polybius_square(&lang, &keyword_str);

        for (i, letter) in input.chars().enumerate() {
            case.push(letter.is_lowercase());

            let idx = polybius
                .iter()
                .position(|&x| x == letter.to_upper_case())
                .unwrap_or(usize::MAX);

            if idx == usize::MAX {
                other.insert(i, letter);
                continue;
            }

            x_cord.push((idx as f64 / size as f64).ceil() as usize - 1);
            y_cord.push(idx % size);
        }

        x_cord.extend(y_cord);
        let mut kludge = false;
        let mut i = 0;
        let mut case_idx = 0;
        let input_len = input.chars().count();

        while output.chars().count() != input_len {
            if let Some(ch) = other.get(&output.chars().count()) {
                output.push(match case[case_idx] {
                    true => ch.to_lower_case(),
                    false => *ch,
                });
                case_idx += 1;
                continue;
            }

            if kludge {
                kludge = !kludge;
                continue;
            }

            let letter = polybius[x_cord[i] * size + x_cord[i + 1]];
            output.push({
                let out = match case[case_idx] {
                    true => letter.to_lower_case(),
                    false => letter,
                };
                case_idx += 1;
                out
            });
            kludge = !kludge;
            i += 2;
        }

        Ok(output)
    }
}

#[derive(Deserialize)]
struct Params {
    lang: SupportedLanguages,
    keyword: String,
}

create_me_daddy!();

/// The Bifid cipher is a cipher which uses a Polybius square in conjunction with transposition, which can be fairly difficult to decipher without knowing the alphabet keyword.
/// <br><br/>
/// For more information about cipher/hash_function go [here](https://wikipedia.org/wiki/Bifid_cipher)
/// For more information about this function go [here](DOCS_URL)
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/BifidCipherEncode with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "lang": SupportedLanguages,
///         "keyword": string
///     }
/// }
/// ```
/// #### where
///     - SupportedLanguages is enum of "en", "ru", "ru_with_yo".
/// <br/><br/>
///
/// ## Server response have two possible formats
///
/// ### &nbsp;&nbsp;&nbsp;&nbsp; Ok variant
/// ``` json
/// { "Ok": `some answer` }
/// ```
/// ### &nbsp;&nbsp;&nbsp;&nbsp; Error variant
/// ``` json
/// { "Err": `error message` }
/// ```
/// # Examples
/// ## №1
/// ``` http
/// POST /api/BifidCipherEncode
///
/// {
///     "input": "hello",
///     "params": {
///         "lang": "en",
///         "keyword": "key"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "cmodh"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/BifidCipherEncode
///
/// {
///     "input": "тестовое сообщение на русском",
///     "params": {
///         "lang": "ru",
///         "keyword": "ключ"
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "ртйксйцк еатслсм,э оя чънрчкю"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/BifidCipherEncode
///
/// {
///     "input": "тестовое сообщение на русском",
///     "params": {
///         "lang": "ru",
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `keyword`."
/// }
/// ```
pub struct BifidCipherEncode;

const NAME: &str = "BifidCipherEncode";
const DESCRIPTION_EN: &str = "The Bifid cipher is a cipher which uses a Polybius square in conjunction with transposition, which can be fairly difficult to decipher without knowing the alphabet keyword.";
const DESCRIPTION_RU: &str = "Шифр Бифида - это шифр, в котором используется квадрат Полибия в сочетании с транспозицией, которую довольно сложно расшифровать, не зная ключевого слова алфавита.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Bifid_cipher");

create_info_struct!(
    BifidCipherEncodeInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
