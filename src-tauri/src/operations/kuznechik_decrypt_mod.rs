use crate::{
    libs::libakrypt::{
        akrypt::Akrypt,
        structs::{AkryptFunction, InputFormat, Mode, OutputFormat},
    },
    run_op, Operation,
};
use anyhow::Result;
use serde::Deserialize;

run_op!(run_kuznechikdecrypt, KuznechikDecrypt);

impl Operation<'_, DeserializeMeDaddy> for KuznechikDecrypt {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let DeserializeMeDaddy {
            key,
            iv,
            mode,
            input_format,
            ..
        } = request;

        let akrypt = Akrypt::new(AkryptFunction::Kuznechik)
            .set_input(&input, input_format)?
            .set_iv(iv.as_bytes())?
            .set_key(key.as_bytes())?
            .set_mode(mode);

        akrypt.decrypt()
    }
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    key: String,
    iv: String,
    mode: Mode,
    input_format: InputFormat,
    _output_format: Option<OutputFormat>,
}

pub struct KuznechikDecrypt;
