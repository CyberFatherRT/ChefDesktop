use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use crate::{
    create_info_struct, create_tauri_wrapper, run_operations, utils::to_hex, Operation, DOCS_URL,
};

create_tauri_wrapper!(sha1, SHA1, String, String);

impl Operation<'_, DeserializeMeDaddy, String> for SHA1 {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let input = request.input;

        let mut hasher = Sha1::new();
        hasher.update(input.as_bytes());
        let result = &hasher.finalize()[..];

        Ok(to_hex(result))
    }
}

#[derive(Deserialize)]
pub struct DeserializeMeDaddy {
    input: String,
}

/// The SHA (Secure Hash Algorithm) hash functions were designed by the NSA. SHA-1 is the most established of the existing SHA hash functions, and it is used in a variety of security applications and protocols. However, SHA-1's collision resistance has been weakening as new attacks are discovered or improved.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/SHA-1).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/SHA1 with your data using json payload with this structure.
/// ``` json
/// {
///     "input": string,
/// }
/// ```
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
/// POST /api/SHA1
///
/// {
///     "input": "hello",
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": aaf4c61ddcc5e8a2dabedef3b482cd9aea9434d"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/SHA1
///
/// {
///     "input": "hello 123 world",
/// }
/// ```
/// ```http
/// {
///   "Ok": "9bb51a436f8faadcbbd793cc96fb1ab38b281"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/SHA1
///
/// {
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `input`"
/// }
/// ```
pub struct SHA1;

const NAME: &str = "SHA1";
const DESCRIPTION_EN: &str = "The SHA (Secure Hash Algorithm) hash functions were designed by the NSA. SHA-1 is the most established of the existing SHA hash functions and it is used in a variety of security applications and protocols. However, SHA-1's collision resistance has been weakening as new attacks are discovered or improved.";
const DESCRIPTION_RU: &str = "Хэш-функции SHA (Secure Hash Algorithm) были разработаны АНБ. SHA-1 самая распространенная из существующих хеш-функций SHA. Она используется в различных приложениях и протоколах безопасности. Однако устойчивость SHA-1 к коллизиям ослабевает по мере обнаружения или улучшения новых атак.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/SHA-1");

create_info_struct!(
    Sha1Info,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
