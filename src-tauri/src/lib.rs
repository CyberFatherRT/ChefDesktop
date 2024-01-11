mod libs;
mod macros;
mod operations;
mod traits;
mod utils;

pub use operations::*;
use serde::{Deserialize, Serialize};
use traits::StringTrait;


pub fn run_operations<'a, I, O>(
    operations: impl Operation<'a, I, O>,
    request: &str,
) -> Result<O, String>
where
    I: Deserialize<'a>,
    O: Serialize,
{
    operations.do_black_magic(request)
}

pub trait Operation<'a, I, O>
where
    I: Deserialize<'a>,
    O: Serialize,
{
    fn do_black_magic(&self, request: &str) -> Result<O, String>;
    fn validate(&self, request: &'a str) -> Result<I, String> {
        self.deserialize(request)
    }

    fn deserialize(&self, request: &'a str) -> Result<I, String> {
        serde_json::from_str(request).map_err(|err| match err.to_string() {
            err if err.starts_with("unknown")
                || err.starts_with("missing")
                || err.starts_with("invalid") =>
            {
                err.split(" at line ")
                    .next()
                    .unwrap()
                    .to_string()
                    .capitalize()
                    + "."
            }
            err => err.capitalize() + ".",
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Operations {
    A1Z26CipherDecode,
    A1Z26CipherEncode,
    Adler32CheckSum,
    AffineCipherDecode,
    AffineCipherEncode,
    AnalyseHash,
    Argon2Compare,
    Argon2,
    AtbashCipher,
    BaconCipherEncode,
    BaconCipherDecode,
    BcryptCompare,
    Bcrypt,
    BcryptParse,
    BifidCipherEncode,
    Blake2b,
    Blake2s,
    FromBase64,
    FromBase,
    HMAC,
    MD2,
    MD4,
    MD5,
    RSADecrypt,
    RSAEncrypt,
    SHA1,
    SHA2,
    SHA3,
    ToBase64,
    ToBase,
    VigenereCipherDecode,
    VigenereCipherEncode,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Hex(String),
    Base64(String),
    Uint8Array(Vec<u8>),
}

pub const DOCS_URL: &str = "soon I transfer all documentation to somewhere :/";
