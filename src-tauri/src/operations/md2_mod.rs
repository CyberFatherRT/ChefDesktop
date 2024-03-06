use anyhow::Result;
use md2::{Digest, Md2};
use serde::Serialize;

use crate::{create_info_struct, utils::to_hex, Operation, DOCS_URL};

impl Operation<'_, ()> for MD2 {
    fn do_black_magic(&self, input: &str, _request: &str) -> Result<String> {
        let mut hasher = Md2::new();
        hasher.update(input);
        let result = hasher.finalize().to_vec();

        Ok(to_hex(&result))
    }
}

/// The MD2 (Message-Digest 2) algorithm is a cryptographic hash function developed by Ronald Rivest in 1989. The algorithm is optimized for 8-bit computers. Although MD2 is no longer considered secure, even as of 2014, it remains in use in public key infrastructures as part of certificates generated with MD2 and RSA.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/MD2_(cryptography)).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/MD2 with your data using json payload with this structure.
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
/// POST /api/MD2
///
/// {
///     "input": "hello",
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": a946c73e0331af68917d384f7655"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/MD2
///
/// {
///     "input": "hello 123 world",
/// }
/// ```
/// ```http
/// {
///   "Ok": "5a594adcc1d94cf325aa35893e49664"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/MD2
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
pub struct MD2;

const NAME: &str = "MD2";
const DESCRIPTION_EN: &str = "The MD2 (Message-Digest 2) algorithm is a cryptographic hash function developed by Ronald Rivest in 1989. The algorithm is optimized for 8-bit computers. Although MD2 is no longer considered secure, even as of 2014, it remains in use in public key infrastructures as part of certificates generated with MD2 and RSA.";
const DESCRIPTION_RU: &str = "Алгоритм MD2 (Message-Digest 2) — это криптографическая хэш-функция, разработанная Рональдом Ривестом в 1989 году. Алгоритм оптимизирован для 8-битных компьютеров. Хотя MD2 больше не считается безопасным, даже с 2014 года он по-прежнему используется в инфраструктурах открытых ключей как часть сертификатов, созданных с помощью MD2 и RSA.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/MD2_(cryptography)");

create_info_struct!(
    Md2Info,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
