use anyhow::Result;
use md4::{Digest, Md4};
use serde::Serialize;

use crate::{create_info_struct, utils::to_hex, Operation, DOCS_URL};

impl Operation<'_, ()> for MD4 {
    fn do_black_magic(&self, input: &str, _request: &str) -> Result<String> {
        let mut hasher = Md4::new();
        hasher.update(input);
        let result = hasher.finalize().to_vec();

        Ok(to_hex(&result))
    }
}

/// The MD4 (Message-Digest 4) algorithm is a cryptographic hash function developed by Ronald Rivest in 1990. The digest length is 128 bits. The algorithm has influenced later designs, such as the MD5, SHA-1 and RIPEMD algorithms. The security of MD4 has been severely compromised.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/MD4).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/MD4 with your data using json payload with this structure.
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
/// POST /api/MD4
///
/// {
///     "input": "hello",
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": 866437cb7a794bce2b727acc362ee27"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/MD4
///
/// {
///     "input": "hello 123 world",
/// }
/// ```
/// ```http
/// {
///   "Ok": "a3766d3b55379e5fbeba5e675db2f3"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/MD4
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
pub struct MD4;

const NAME: &str = "MD4";
const DESCRIPTION_EN: &str = "The MD4 (Message-Digest 4) algorithm is a cryptographic hash function developed by Ronald Rivest in 1990. The digest length is 128 bits. The algorithm has influenced later designs, such as the MD5, SHA-1 and RIPEMD algorithms. The security of MD4 has been severely compromised.";
const DESCRIPTION_RU: &str = "Алгоритм MD4 (Message-Digest 4) — это криптографическая хэш-функция, разработанная Рональдом Ривестом в 1990 году. Длина дайджеста составляет 128 бит. Алгоритм повлиял на более поздние разработки, такие как алгоритмы MD5, SHA-1 и RIPEMD. Безопасность MD4 была серьезно скомпрометирована».";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/MD4");

create_info_struct!(
    Md4Info,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
