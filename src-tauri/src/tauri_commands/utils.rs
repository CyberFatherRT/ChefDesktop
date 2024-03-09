use chef_desktop::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeOperations {
    name: Operations,
    request: String,
}

#[tauri::command]
pub fn gsd(mut input: String, ops: Vec<RecipeOperations>) -> Result<String, String> {
    println!("{ops:?}");
    for op in ops {
        input = match op.name {
            Operations::A1Z26CipherDecode => {
                run_a1z26cipherdecode(A1Z26CipherDecode, &input, &op.request)?
            }
            Operations::A1Z26CipherEncode => {
                run_a1z26cipherencode(A1Z26CipherEncode, &input, &op.request)?
            }
            Operations::Adler32CheckSum => {
                run_adler32checksum(Adler32CheckSum, &input, &op.request)?
            }
            Operations::AffineCipherDecode => {
                run_affinecipherdecode(AffineCipherDecode, &input, &op.request)?
            }
            Operations::AffineCipherEncode => {
                run_affinecipherencode(AffineCipherEncode, &input, &op.request)?
            }
            Operations::AnalyseHash => run_analysehash(AnalyseHash, &input, &op.request)?,
            Operations::Argon2Compare => run_argon2compare(Argon2Compare, &input, &op.request)?,
            Operations::Argon2 => run_argon2(Argon2, &input, &op.request)?,
            Operations::AtbashCipher => run_atbashcipher(AtbashCipher, &input, &op.request)?,
            Operations::BaconCipherEncode => {
                run_baconcipherencode(BaconCipherEncode, &input, &op.request)?
            }
            Operations::BaconCipherDecode => {
                run_baconcipherdecode(BaconCipherDecode, &input, &op.request)?
            }
            Operations::BcryptCompare => run_bcryptcompare(BcryptCompare, &input, &op.request)?,
            Operations::Bcrypt => run_bcrypt(Bcrypt, &input, &op.request)?,
            Operations::BcryptParse => run_bcryptparse(BcryptParse, &input, &op.request)?,
            Operations::BifidCipherEncode => {
                run_bifidcipherencode(BifidCipherEncode, &input, &op.request)?
            }
            Operations::Blake2b => run_blake2b(Blake2b, &input, &op.request)?,
            Operations::Blake2s => run_blake2s(Blake2s, &input, &op.request)?,
            Operations::FromBase64 => run_frombase64(FromBase64, &input, &op.request)?,
            Operations::FromBase => run_frombase(FromBase, &input, &op.request)?,
            Operations::HMAC => run_hmac(HMAC, &input, &op.request)?,
            Operations::KuznechikDecrypt => {
                run_kuznechikdecrypt(KuznechikDecrypt, &input, &op.request)?
            }
            Operations::KuznechikEncrypt => {
                run_kuznechikencrypt(KuznechikEncrypt, &input, &op.request)?
            }
            Operations::MagmaDecrypt => run_magmadecrypt(MagmaDecrypt, &input, &op.request)?,
            Operations::MagmaEncrypt => run_magmaencrypt(MagmaEncrypt, &input, &op.request)?,
            Operations::MD2 => run_md2(MD2, &input, &op.request)?,
            Operations::MD4 => run_md4(MD4, &input, &op.request)?,
            Operations::MD5 => run_md5(MD5, &input, &op.request)?,
            Operations::RSADecrypt => run_rsadecrypt(RSADecrypt, &input, &op.request)?,
            Operations::RSAEncrypt => run_rsaencrypt(RSAEncrypt, &input, &op.request)?,
            Operations::Scrypt => run_scrypt(Scrypt, &input, &op.request)?,
            Operations::SHA1 => run_sha1(SHA1, &input, &op.request)?,
            Operations::SHA2 => run_sha2(SHA2, &input, &op.request)?,
            Operations::SHA3 => run_sha3(SHA3, &input, &op.request)?,
            Operations::ToBase64 => run_tobase64(ToBase64, &input, &op.request)?,
            Operations::ToBase => run_tobase(ToBase, &input, &op.request)?,
            Operations::VigenereCipherDecode => {
                run_vigenerecipherdecode(VigenereCipherDecode, &input, &op.request)?
            }
            Operations::VigenereCipherEncode => {
                run_vigenerecipherencode(VigenereCipherEncode, &input, &op.request)?
            }
        };
    }

    Ok(input)
}
