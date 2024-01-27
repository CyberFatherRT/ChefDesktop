use anyhow::{bail, Result};
use rsa::{pkcs1::DecodeRsaPublicKey, Oaep, Pkcs1v15Encrypt, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};

use crate::{
    create_info_struct, create_me_daddy, create_tauri_wrapper, libs::base64::to_base64,
    run_operations, utils::to_hex, Operation, DOCS_URL,
};

create_tauri_wrapper!(rsa_encrypt, RSAEncrypt);

impl Operation<'_, DeserializeMeDaddy> for RSAEncrypt {
    fn do_black_magic(&self, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let (input, public_key, encrypted_scheme, message_digest_algorithm, output_format) = (
            request.input,
            request.params.public_key,
            request.params.encrypted_scheme,
            request.params.message_digest_algorithm,
            request.params.output_format,
        );

        if matches!(encrypted_scheme, SupportedEncryptionSchemes::RSA_OAEP)
            && message_digest_algorithm.is_none()
        {
            bail!("RSA_OAEP must have message digest algorithm");
        }

        let pub_key = RsaPublicKey::from_pkcs1_pem(&public_key)?;
        let mut rng = rand::thread_rng();

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
                pub_key.encrypt(&mut rng, padding, input.as_bytes())
            }
            SupportedEncryptionSchemes::RSA_AES_PKCS1_V1_5 => {
                pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, input.as_bytes())
            }
        }?;

        Ok(match output_format {
            SupportedOutputFormat::Hex => to_hex(&encrypted_text),
            SupportedOutputFormat::Base64 => to_base64(&encrypted_text, None)?,
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
struct Params {
    #[serde(rename = "pub_key")]
    public_key: String,
    #[serde(rename = "scheme")]
    encrypted_scheme: SupportedEncryptionSchemes,
    #[serde(rename = "digest_alg")]
    message_digest_algorithm: Option<SupportedMessageDigestAlgorithm>,
    output_format: SupportedOutputFormat,
}

create_me_daddy!();

/// Encrypt a message with a PEM encoded RSA public key.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/RSA_(cryptosystem))
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/RSAEncrypt with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "pub_key": PEM,
///         "scheme": SupportedEncryptionSchemes,
///         "digest_alg": Option<SupportedMessageDigestAlgorithm>
///         "output_format": SupportedOutputFormat
///     }
/// }
/// ```
/// #### where
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
/// POST /api/RSAEncrypt
///
/// {
///     "input": "hello",
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
///     "hex": "66bbd6ab6373e0564c4e7adbbc9dd27a9ca4a0857254c7ea00d62a5cbd21eaeacd9684bbcc7a7922260ea686187f41eb6befe117f09c46343e57260dc1f4b4d80ccc0cfc87f2d0ce836ee6a7326a94f2ace2ca1f76c3139966237fc97c3abe8251ef4733266855f3d5174b1796524ce5f419d25d79b856113517c5c933f2d1dce37bd0d5783b384ee9c17b2562a34da964bff799d6152a163f0e9455f1fe5f488c02c46373be3b4cf388b6a04aa4354fd094918b7d98f3351b6e8d575816e542a72d03085cddd9f7d79f886304934a7474ce2c019382cf217b632e170ed286b9ee0f956ff12f93e64af4c20cae4a69c91a356e4ffbce6531"
///   }
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/RSAEncrypt
///
/// {
///     "input": "hello",
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
///     "base64": "kip548Yss3HwbrlOJHwtVhRn6tOfEyFcO5UQ4Otgx4HLI2m89mOTEgeo6qH8tkgPfZsno8aTHXaXH+UUoV3WnNF5Q5Y0/Fql2APQYQ3dsaU3sdnzDR/dX4yaGhfOZ3hhbalo509mbqR5kz27VcAhgmXPZIKHGTwjra/hjYhyP3uWqxw/svsxPyIZqOz8Y0qb84GIjNk0+5cFRAJMuV6+fpme5UyApHpaw4GVY9XxiNKECkQ+etK0jr/aclLjiU7I60Qayrhstlf10l8SYgRtmvs6TTVmYYGbRDwLNe9CqPCaqGUsPQpLtP8GVVEKE7M/RNemQgKfvZIfj/N6b3W58g="
///   }
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/RSAEncrypt
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
///   "Err": "Missing field `pub_key`"
/// }
/// ```
pub struct RSAEncrypt;

const NAME: &str = "RSAEncrypt";
const DESCRIPTION_EN: &str = "Encrypt a message with a PEM encoded RSA public key.";
const DESCRIPTION_RU: &str = "Шифрует сообщение с помощью открытого ключа RSA с кодировкой PEM.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/RSA_(cryptosystem)");

create_info_struct!(
    RSAEncryptInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
