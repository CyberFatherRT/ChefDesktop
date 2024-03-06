use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

use crate::{create_info_struct, run_op, utils::to_hex, Operation, DOCS_URL};

run_op!(run_sha2, SHA2);

impl Operation<'_, DeserializeMeDaddy> for SHA2 {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let size = request.size;

        let result = match size {
            SupportedSHA2Size::SHA224 => {
                let mut hasher = Sha224::new();
                hasher.update(input);
                to_hex(&hasher.finalize()[..])
            }
            SupportedSHA2Size::SHA256 => {
                let mut hasher = Sha256::new();
                hasher.update(input);
                to_hex(&hasher.finalize()[..])
            }
            SupportedSHA2Size::SHA384 => {
                let mut hasher = Sha384::new();
                hasher.update(input);
                to_hex(&hasher.finalize()[..])
            }
            SupportedSHA2Size::SHA512 => {
                let mut hasher = Sha512::new();
                hasher.update(input);
                to_hex(&hasher.finalize()[..])
            }
            SupportedSHA2Size::SHA512_224 => {
                let mut hasher = Sha512_224::new();
                hasher.update(input);
                to_hex(&hasher.finalize()[..])
            }
            SupportedSHA2Size::SHA512_256 => {
                let mut hasher = Sha512_256::new();
                hasher.update(input);
                to_hex(&hasher.finalize()[..])
            }
        };

        Ok(result)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum SupportedSHA2Size {
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    SHA512_224,
    SHA512_256,
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    size: SupportedSHA2Size,
}

/// The SHA-2 (Secure Hash Algorithm 2) hash functions were designed by the NSA. SHA-2 includes significant changes from its predecessor, SHA-1. The SHA-2 family consists of hash functions with digests (hash values) that are 224, 256, 384 or 512 bits: SHA224, SHA256, SHA384, SHA512. SHA-512 operates on 64-bit words. SHA-256 operates on 32-bit words. SHA-384 is largely identical to SHA-512 but is truncated to 384 bytes. SHA-224 is largely identical to SHA-256 but is truncated to 224 bytes. SHA-512/224 and SHA-512/256 are truncated versions of SHA-512, but the initial values are generated using the method described in Federal Information Processing Standards (FIPS) PUB 180-4
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/SHA-2).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/SHA2 with your data using json payload with this structure.
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
/// POST /api/SHA2
///
/// {
///     "input": "hello",
///     "params": {
///         "size": "sha512"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": 9b71d224bd62f3785d96d46ad3ea3d73319bfbc289caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5dac4663475c2e5c3adef46f73bcdec043"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/SHA2
///
/// {
///     "input": "hello 123 world",
///     "params": {
///         "size": "sha224"
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "1d5d7d218ccaef8b16a442970f8c56a31394126c053ef2cd8ed7f3"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/SHA2
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
pub struct SHA2;

const NAME: &str = "SHA2";
const DESCRIPTION_EN: &str = "The SHA-2 (Secure Hash Algorithm 2) hash functions were designed by the NSA. SHA-2 includes significant changes from its predecessor, SHA-1. The SHA-2 family consists of hash functions with digests (hash values) that are 224, 256, 384 or 512 bits: SHA224, SHA256, SHA384, SHA512. SHA-512 operates on 64-bit words. SHA-256 operates on 32-bit words. SHA-384 is largely identical to SHA-512 but is truncated to 384 bytes. SHA-224 is largely identical to SHA-256 but is truncated to 224 bytes. SHA-512/224 and SHA-512/256 are truncated versions of SHA-512, but the initial values are generated using the method described in Federal Information Processing Standards (FIPS) PUB 180-4";
const DESCRIPTION_RU: &str = "Хэш-функции SHA-2 (Secure Hash Algorithm 2) были разработаны АНБ. SHA-2 включает в себя существенные отличия от своего предшественника SHA-1. Семейство SHA-2 состоит из хеш-функций с дайджестами (хеш-значениями) размером 224, 256, 384 или 512 бит: SHA224, SHA256, SHA384, SHA512. SHA-512 работает с 64-битными словами. SHA-256 работает с 32-битными словами. SHA-384 во многом идентичен SHA-512, но усечен до 384 байт. SHA-224 во многом идентичен SHA-256, но усечен до 224 байт. SHA-512/224 и SHA-512/256 являются усеченными версиями SHA-512, но начальные значения генерируются с использованием метода, описанного в Федеральных стандартах обработки информации (FIPS) PUB 180-4.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/SHA-2");

create_info_struct!(
    Sha2Info,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
