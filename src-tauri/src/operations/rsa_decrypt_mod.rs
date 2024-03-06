use anyhow::{bail, Result};
use rsa::{pkcs1::DecodeRsaPrivateKey, Oaep, Pkcs1v15Encrypt, RsaPrivateKey};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};

use crate::{
    create_info_struct,
    libs::base64::{from_base64, to_base64},
    run_op,
    utils::to_hex,
    Operation, DOCS_URL,
};

run_op!(run_rsadecrypt, RSADecrypt);

impl Operation<'_, DeserializeMeDaddy> for RSADecrypt {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;

        let DeserializeMeDaddy {
            private_key,
            encrypted_scheme,
            message_digest_algorithm,
            output_format,
            input_format,
        } = request;

        if matches!(encrypted_scheme, SupportedEncryptionSchemes::RSA_OAEP)
            && message_digest_algorithm.is_none()
        {
            bail!("RSA_OAEP must have message digest algorithm");
        }

        let input = match input_format {
            SupportedOutputFormat::Hex => hex::decode(input)?,
            SupportedOutputFormat::Base64 => from_base64(&input)?,
            SupportedOutputFormat::Raw => input.as_bytes().to_vec(),
        };

        let pem_key: RsaPrivateKey = DecodeRsaPrivateKey::from_pkcs1_pem(&private_key)?;

        let encrypted_text = match encrypted_scheme {
            SupportedEncryptionSchemes::RSA_OAEP => {
                let padding = match message_digest_algorithm.unwrap() {
                    SupportedMessageDigestAlgorithm::SHA1 => Oaep::new::<Sha1>(),
                    SupportedMessageDigestAlgorithm::SHA2_224 => Oaep::new::<Sha224>(),
                    SupportedMessageDigestAlgorithm::SHA2_256 => Oaep::new::<Sha256>(),
                    SupportedMessageDigestAlgorithm::SHA2_384 => Oaep::new::<Sha384>(),
                    SupportedMessageDigestAlgorithm::SHA2_512 => Oaep::new::<Sha512>(),
                    SupportedMessageDigestAlgorithm::SHA3_224 => Oaep::new::<Sha3_224>(),
                    SupportedMessageDigestAlgorithm::SHA3_256 => Oaep::new::<Sha3_256>(),
                    SupportedMessageDigestAlgorithm::SHA3_384 => Oaep::new::<Sha3_384>(),
                    SupportedMessageDigestAlgorithm::SHA3_512 => Oaep::new::<Sha3_512>(),
                };
                pem_key.decrypt(padding, &input)
            }
            SupportedEncryptionSchemes::RSA_AES_PKCS1_V1_5 => {
                pem_key.decrypt(Pkcs1v15Encrypt, &input)
            }
        }?;

        Ok(match output_format {
            SupportedOutputFormat::Hex => to_hex(&encrypted_text),
            SupportedOutputFormat::Base64 => to_base64(&encrypted_text),
            SupportedOutputFormat::Raw => String::from_utf8_lossy(&encrypted_text).to_string(),
        })
    }
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum SupportedEncryptionSchemes {
    #[serde(rename = "oaep")]
    RSA_OAEP,
    #[serde(rename = "pkcs1_v15")]
    RSA_AES_PKCS1_V1_5,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum SupportedMessageDigestAlgorithm {
    SHA1,
    SHA2_224,
    SHA2_256,
    SHA2_384,
    SHA2_512,
    SHA3_224,
    SHA3_256,
    SHA3_384,
    SHA3_512,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum SupportedOutputFormat {
    Hex,
    Base64,
    Raw,
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    #[serde(rename = "pem_key")]
    private_key: String,
    #[serde(rename = "scheme")]
    encrypted_scheme: SupportedEncryptionSchemes,
    #[serde(rename = "digest_alg")]
    message_digest_algorithm: Option<SupportedMessageDigestAlgorithm>,
    output_format: SupportedOutputFormat,
    input_format: SupportedOutputFormat,
}

/// Decrypt a message with a PEM encoded RSA private key.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/RSA_(cryptosystem))
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/RSADecrypt with your data using json payload with this structure
/// ``` json
/// {
///     "input": base64,
///     "params": {
///         "pem_key": PEM,
///         "scheme": SupportedEncryptionSchemes,
///         "digest_alg": Option<SupportedMessageDigestAlgorithm>
///         "output_format": SupportedOutputFormat
///     }
/// }
/// ```
/// #### where
///     - base64 is base64 encoded string
///     - PEM is pem encoded RSA public key
///     - SupportedEncryptionSchemes is enum of "oaep" and "pkcs1_v15"
///     - Option<SupportedMessageDigestAlgorithm> is optional enum of "sha1", "sha2-224", "sha2-256", "sha2-384", "sha2-512", "sha3-224", "sha3-256", "sha3-384", "sha3-512"
///     - SupportedOutputFormat is enum of "hex", "base64", "uint8array"
/// <br/><br/>
///
/// ### Server response have two possible formats
///
/// #### &nbsp;&nbsp;&nbsp;&nbsp; Ok variant
/// ``` json
/// {
///   "Ok": {
///     "hex|base64|uint8array": "string|uint8array"
///   }
/// }
/// ```
/// #### &nbsp;&nbsp;&nbsp;&nbsp; Error variant
/// ``` json
/// { "Err": `error message` }
/// ```
/// # Examples
/// ## №1
/// ``` http
/// POST /api/RSADecrypt
///
/// {
///     "input": {base64 encoded input},
///     "params": {
///         "pub_key": {PEM encoded key},
///         "scheme": "oaep",
///         "digest_alg": "sha2_256",
///         "output_format": "hex"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": {
///     "hex": "6e6576657220676f6e6e612e2e"
///   }
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/RSADecrypt
///
/// {
///     "input": {base64 encoded input},
///     "params": {
///         "pub_key": {PEM encoded key},
///         "scheme": "pkcs1_v15",
///         "output_format": "base64"
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": {
///     "base64": "aGVsbG8gd29ybGQK"
///   }
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/RSADecrypt
///
/// {
///     "input": "error",
///     "params": {
///         "scheme": "pkcs1_v15",
///         "output_format": "uint8array"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `pem_key`"
/// }
/// ```
pub struct RSADecrypt;

const NAME: &str = "RSADecrypt";
const DESCRIPTION_EN: &str = "Decrypt a message with a PEM encoded RSA private key.";
const DESCRIPTION_RU: &str = "Дешифрует сообщение с помощью приватного ключа RSA с кодировкой PEM.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/RSA_(cryptosystem)");

create_info_struct!(
    RSADecryptInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
