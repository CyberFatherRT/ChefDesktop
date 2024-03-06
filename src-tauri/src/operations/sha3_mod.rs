use crate::{create_info_struct, utils::to_hex, Operation, DOCS_URL};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};

impl Operation<'_, DeserializeMeDaddy> for SHA3 {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let size = request.size;

        let result = match size {
            SupportedSHA3Size::SHA224 => {
                let mut hasher = Sha3_224::new();
                hasher.update(input);
                to_hex(&hasher.finalize()[..])
            }
            SupportedSHA3Size::SHA256 => {
                let mut hasher = Sha3_256::new();
                hasher.update(input);
                to_hex(&hasher.finalize()[..])
            }
            SupportedSHA3Size::SHA384 => {
                let mut hasher = Sha3_384::new();
                hasher.update(input);
                to_hex(&hasher.finalize()[..])
            }
            SupportedSHA3Size::SHA512 => {
                let mut hasher = Sha3_512::new();
                hasher.update(input);
                to_hex(&hasher.finalize()[..])
            }
        };

        Ok(result)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum SupportedSHA3Size {
    SHA224,
    SHA256,
    SHA384,
    SHA512,
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    size: SupportedSHA3Size,
}

/// The SHA-3 (Secure Hash Algorithm 3) hash functions were released by NIST on August 5, 2015. Although part of the same series of standards, SHA-3 is internally quite different from the MD5-like structure of SHA-1 and SHA-2. SHA-3 is a subset of the broader cryptographic primitive family Keccak designed by Guido Bertoni, Joan Daemen, Michaël Peeters, and Gilles Van Assche, building upon RadioGatún.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/SHA-3).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/SHA3 with your data using json payload with this structure.
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "size": SupportedSHA2Size
///     }
/// }
/// ```
/// #### where
///     - SupportedSHA2Size is enum of "sha224", "sha256", "sha384", "sha512", "sha512_224", "sha512_256"
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
/// POST /api/SHA3
///
/// {
///     "input": "hello",
///     "params": {
///         "size": "sha224"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": b87f88c7272fff1748e58b87e9141a42cdbedc29a78cb0d4a5cd81"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/SHA3
///
/// {
///     "input": "hello 123 world",
///     "params": {
///         "size": "sha384"
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "061c227e47b9eb83b165c6f86de608287fb5e1639af52ed1ccf371da91561925e8d5999f5c8f5a921c7e593325c"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/SHA3
///
/// {
///     "input": "missing params"
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `params`"
/// }
/// ```
pub struct SHA3;

const NAME: &str = "SHA3";
const DESCRIPTION_EN: &str = "The SHA-3 (Secure Hash Algorithm 3) hash functions were released by NIST on August 5, 2015. Although part of the same series of standards, SHA-3 is internally quite different from the MD5-like structure of SHA-1 and SHA-2. SHA-3 is a subset of the broader cryptographic primitive family Keccak designed by Guido Bertoni, Joan Daemen, Michaël Peeters, and Gilles Van Assche, building upon RadioGatún.";
const DESCRIPTION_RU: &str = "Хэш-функции SHA-3 (Secure Hash Algorithm 3) были выпущены NIST 5 августа 2015 года. Хотя SHA-3 является частью той же серии стандартов, внутренне он сильно отличается от MD5-подобной структуры SHA-1 и SHA. -2. SHA-3 — это подмножество более широкого семейства криптографических примитивов Keccak, разработанного Гвидо Бертони, Джоан Демен, Микаэлем Петерсом и Жилем Ван Аше на основе RadioGatún.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/SHA-3");

create_info_struct!(
    Sha3Info,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
