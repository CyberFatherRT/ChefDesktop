mod libs;
mod macros;
mod operations;
mod traits;
mod utils;

use anyhow::{Error, Result};
pub use operations::*;
use serde::{Deserialize, Serialize};
use traits::StringTrait;

// pub fn run_operations<'a, I>(operations: impl Operation<'a, I>, request: &str) -> Result<String>
// where
//     I: Deserialize<'a>,
// {
//     operations.do_black_magic(request)
// }

pub trait Operation<'a, I>
where
    I: Deserialize<'a>,
{
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String>;
    fn validate(&self, request: &'a str) -> Result<I> {
        self.deserialize(request)
    }

    fn deserialize(&self, request: &'a str) -> Result<I> {
        serde_json::from_str(request).map_err(|err| {
            Error::msg(match err.to_string() {
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
    KuznechikDecrypt,
    KuznechikEncrypt,
    MagmaDecrypt,
    MagmaEncrypt,
    MD2,
    MD4,
    MD5,
    RSADecrypt,
    RSAEncrypt,
    Scrypt,
    SHA1,
    SHA2,
    SHA3,
    ToBase64,
    ToBase,
    VigenereCipherDecode,
    VigenereCipherEncode,
}

pub const DOCS_URL: &str = "soon I transfer all documentation to somewhere :/";
