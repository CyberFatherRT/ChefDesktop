use serde::Deserialize;
use anyhow::Result;
use crate::{create_me_daddy, create_tauri_wrapper, libs::libakrypt::{akrypt::Akrypt, structs::{AkryptFunction, InputFormat, Mode, OutputFormat}}, run_operations, Operation};

create_tauri_wrapper!(kuznechik, Kuznechik);

impl Operation<'_, DeserializeMeDaddy> for Kuznechik {
    fn do_black_magic(&self, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let (input, Params { key, iv, mode, input_format, output_format }) = (request.input, request.params);

        let akrypt = Akrypt::new(AkryptFunction::Kuznechik)
            .set_input(&input, input_format)?
            .set_iv(iv.as_bytes())?
            .set_key(key.as_bytes())?
            .set_mode(mode);

        akrypt.encrypt(output_format)
    }
}

#[derive(Deserialize)]
pub struct Params {
    key: String,
    iv: String,
    mode: Mode,
    input_format: InputFormat,
    output_format: OutputFormat,
}

create_me_daddy!();

pub struct Kuznechik;