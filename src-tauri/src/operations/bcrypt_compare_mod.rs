use crate::{
    create_info_struct, create_me_daddy, create_tauri_wrapper, run_operations, Operation, DOCS_URL,
};
use serde::{Deserialize, Serialize};

create_tauri_wrapper!(bcrypt_compare, BcryptCompare, String, String);

impl Operation<'_, DeserializeMeDaddy, String> for BcryptCompare {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (input, encoded_hash) = (request.input, request.params.encoded_hash);

        let res = bcrypt::verify(input.as_bytes(), &encoded_hash).map_err(|err| err.to_string())?;

        match res {
            true => Ok(format!("Match `{}`.", input)),
            false => Err("No match.".to_string()),
        }
    }
}

#[derive(Deserialize)]
struct Params {
    encoded_hash: String,
}

create_me_daddy!();

/// Tests whether the input matches the given bcrypt hash. To test multiple possible passwords, use the 'Fork' operation.
/// <br><br/>
/// For more information about cipher/hash_function go [here](https://wikipedia.org/wiki/Bcrypt)\
/// For more information about this function go [here](DOCS_URL)
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/BcryptCompare with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "encoded_hash": String
///     }
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
/// POST /api/BcryptCompare
///
/// {
///     "input": "hello",
///     "params": {
///         "encoded_hash": "$2b$12$mLDUe/nTaPt06W2ai4YrVeCiPK7/L1Dhj7FipakSCnKIDsgqbvPgm"
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
/// POST /api/BcryptCompare
///
/// {
///     "input": "Привет, Мир!",
///     "params": {
///         "encoded_hash": "$2x$04$hC6BHE9hPEQZExczLDTxBOgq48yNMI7HC5bmE0HiP/iGxtMpwryh6"
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
/// POST /api/BcryptCompare
///
/// {
///     "input": "missing encoded_hash",
///     "params": {
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `encoded_hash`."
/// }
/// ```

pub struct BcryptCompare;

const NAME: &str = "BcryptCompare";
const DESCRIPTION_EN: &str = "Tests whether the input matches the given bcrypt hash. To test multiple possible passwords, use the 'Fork' operation.";
const DESCRIPTION_RU: &str = "Проверяет, соответствует ли ввод заданному хешу bcrypt. Чтобы протестировать несколько возможных паролей, используйте операцию `Fork`.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Bcrypt");

create_info_struct!(
    BcryptCompareInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
