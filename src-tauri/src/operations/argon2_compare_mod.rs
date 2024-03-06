use crate::{create_info_struct, Operation, DOCS_URL};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy> for Argon2Compare {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let encoded_hash = request.encoded_hash;

        let res = argon2::verify_encoded(&encoded_hash, input.as_bytes())?;

        match res {
            true => Ok(format!("Match `{}`.", input)),
            false => Err(anyhow!("No match.")),
        }
    }
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    encoded_hash: String,
}

/// Tests whether the input matches the given Argon2 hash. To test multiple possible passwords, use the 'Fork' operation.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Argon2)
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/Argon2Compare with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "encoded_hash": EncodedHash,
///     }
/// }
/// ```
/// #### where
///     - EncodedHash is argon2 hash which look like that $argon2i$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$WVDOfucSPAey3UEzzqLtBwRbGS83pTyIPLXgjhKfgrY
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
/// POST /api/Argon2Compare
///
/// {
///     "input": "hello",
///     "params": {
///         "encoded_hash": "$argon2i$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$WVDOfucSPAey3UEzzqLtBwRbGS83pTyIPLXgjhKfgrY",
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "Match `hello`."
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/Argon2Compare
///
/// {
///     "input": "Привет, Мир!",
///     "params": {
///         "encoded_hash": "$argon2id$v=19$m=8096,t=6,p=1$0L3QvtCy0LDRjyDRgdC+0LvRjA$60FAt47RxPzXNsG3PN9VW6JENx/1OXHlOCPkEa7vvWB1HQ",
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "Match `Привет, Мир!`."
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/Argon2Compare
///
/// {
///     "input": "error",
///     "params": {
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `encoded_hash`"
/// }
/// ```
pub struct Argon2Compare;

const NAME: &str = "Argon2compare";
const DESCRIPTION_EN: &str = "Tests whether the input matches the given Argon2 hash. To test multiple possible passwords, use the 'Fork' operation.";
const DESCRIPTION_RU: &str = "Проверяет соответствие входных данных заданному хешу Argon2. Чтобы протестировать несколько возможных паролей, используйте операцию `Fork`.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Argon2");

create_info_struct!(
    Argon2CompareInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
