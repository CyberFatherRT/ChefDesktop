use crate::{create_info_struct, run_op, Operation, DOCS_URL};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

run_op!(run_bcryptcompare, BcryptCompare);

impl Operation<'_, DeserializeMeDaddy> for BcryptCompare {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let encoded_hash = request.encoded_hash;

        let res = bcrypt::verify(input.as_bytes(), &encoded_hash)?;

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
