use crate::{
    libs::libakrypt::{
        akrypt::Akrypt,
        structs::{AkryptFunction, InputFormat, Mode, OutputFormat},
    },
    run_op, Operation,
};
use anyhow::Result;
use serde::Deserialize;

run_op!(run_kuznechikencrypt, KuznechikEncrypt);

impl Operation<'_, DeserializeMeDaddy> for KuznechikEncrypt {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let DeserializeMeDaddy {
            key,
            iv,
            mode,
            input_format,
            output_format,
        } = request;

        let akrypt = Akrypt::new(AkryptFunction::Kuznechik)
            .set_input(&input, input_format)?
            .set_iv(iv.as_bytes())?
            .set_key(key.as_bytes())?
            .set_mode(mode);

        akrypt.encrypt(output_format)
    }
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    key: String,
    iv: String,
    mode: Mode,
    input_format: InputFormat,
    output_format: OutputFormat,
}

pub struct KuznechikEncrypt;
