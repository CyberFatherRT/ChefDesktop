use crate::{create_info_struct, run_op, Operation, DOCS_URL};
use anyhow::{bail, Result};
use bcrypt::Version;
use serde::{Deserialize, Serialize};

run_op!(run_bcrypt, Bcrypt);

impl Operation<'_, DeserializeMeDaddy> for Bcrypt {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let DeserializeMeDaddy { rounds, version } = request;
        let res = bcrypt::hash_with_result(input.as_bytes(), rounds)?;
        Ok(res.format_for_version(version))
    }

    fn validate(&self, request: &'_ str) -> Result<DeserializeMeDaddy> {
        let request = self.deserialize(request)?;

        if !(4..=31).contains(&request.rounds) {
            bail!("ERROR: Param `rounds` must be between 4 and 31");
        }

        Ok(request)
    }
}

#[derive(Deserialize)]
#[serde(remote = "Version")]
pub enum MyVersion {
    #[serde(rename = "2a")]
    TwoA,
    #[serde(rename = "2x")]
    TwoX,
    #[serde(rename = "2y")]
    TwoY,
    #[serde(rename = "2b")]
    TwoB,
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    rounds: u32,
    #[serde(with = "MyVersion")]
    version: Version,
}

/// bcrypt is a password hashing function designed by Niels Provos and David Mazières, based on the Blowfish cipher, and presented at USENIX in 1999. Besides incorporating a salt to protect against rainbow table attacks, bcrypt is an adaptive function: over time, the iteration count (rounds) can be increased to make it slower, so it remains resistant to brute-force search attacks even with increasing computation power.
/// <br><br/>
/// For more information about cipher/hash_function go [here](https://wikipedia.org/wiki/Bcrypt)
/// For more information about this function go [here](DOCS_URL)
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/Bcrypt with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "rounds": i31,
///         "version": Version
///     }
/// }
/// ```
/// #### where
///     - i31 is signed digit between 4 and 31
///     - Version is enum of "2a", "2x", "2y", "2b"
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
/// POST /api/Bcrypt
///
/// {
///     "input": "hello",
///     "params": {
///         "rounds": 12,
///         "version": "2b"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "$2b$12$mLDUe/nTaPt06W2ai4YrVeCiPK7/L1Dhj7FipakSCnKIDsgqbvPgm"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/Bcrypt
///
/// {
///     "input": "Привет, Мир!",
///     "params": {
///         "rounds": 4,
///         "version": "2x"
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "$2x$04$hC6BHE9hPEQZExczLDTxBOgq48yNMI7HC5bmE0HiP/iGxtMpwryh6"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/Bcrypt
///
/// {
///     "input": "missing version",
///     "params": {
///         "rounds": 4
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `version`."
/// }
/// ```
pub struct Bcrypt;

const NAME: &str = "Bcrypt";
const DESCRIPTION_EN: &str = "bcrypt is a password hashing function designed by Niels Provos and David Mazières, based on the Blowfish cipher, and presented at USENIX in 1999. Besides incorporating a salt to protect against rainbow table attacks, bcrypt is an adaptive function: over time, the iteration count (rounds) can be increased to make it slower, so it remains resistant to brute-force search attacks even with increasing computation power.";
const DESCRIPTION_RU: &str = "bcrypt — это функция хеширования паролей, разработанная Нильсом Провосом и Давидом Мазьером на основе шифра Blowfish и представленная на USENIX в 1999 году. Помимо включения соли для защиты от RainbowTableAttack, bcrypt является адаптивной функцией: со временем количество итераций (раундов) может быть увеличено, чтобы сделать его медленнее, поэтому он остается устойчивым к поисковым атакам методом грубой силы даже при увеличении вычислительной мощности.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Bcrypt");

create_info_struct!(
    BcryptInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
