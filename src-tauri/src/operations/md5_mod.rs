use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};

use crate::{
    create_info_struct, create_tauri_wrapper, run_operations, utils::to_hex, Operation, DOCS_URL,
};

create_tauri_wrapper!(md5, MD5, String, String);

impl Operation<'_, DeserializeMeDaddy, String> for MD5 {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let input = request.input;

        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());
        let result = &hasher.finalize()[..];

        Ok(to_hex(result))
    }
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    input: String,
}

/// MD5 (Message-Digest 5) is a widely used hash function. It has been used in a variety of security applications and is also commonly used to check the integrity of files. However, MD5 is not collision resistant, and it isn't suitable for applications like SSL/TLS certificates or digital signatures that rely on this property.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/MD5).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/MD5 with your data using json payload with this structure.
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
/// POST /api/MD5
///
/// {
///     "input": "hello",
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": 5d41402abc4b2a76b9719d911017c592"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/MD5
///
/// {
///     "input": "hello 123 world",
/// }
/// ```
/// ```http
/// {
///   "Ok": "bdcf33e6dce868a580cbc6686476c4e7"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/MD5
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
pub struct MD5;

const NAME: &str = "MD5";
const DESCRIPTION_EN: &str = "MD5 (Message-Digest 5) is a widely used hash function. It has been used in a variety of security applications, and is also commonly used to check the integrity of files. However, MD5 is not collision resistant and it isn't suitable for applications like SSL/TLS certificates or digital signatures that rely on this property.";
const DESCRIPTION_RU: &str = "MD5 (Message-Digest 5) — широко используемая хэш-функция. Он использовался в различных приложениях безопасности, а также часто используется для проверки целостности файлов. Однако MD5 не устойчив к коллизиям и не подходит для таких приложений, как SSL/TLS-сертификаты или цифровые сертификаты, которые полагаются на это свойство.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/MD5");

create_info_struct!(
    Md5Info,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
