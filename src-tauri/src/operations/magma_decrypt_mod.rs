use crate::{
    create_me_daddy, create_tauri_wrapper,
    libs::libakrypt::{
        akrypt::Akrypt,
        structs::{AkryptFunction, InputFormat, Mode, OutputFormat},
    },
    run_operations, Operation,
};
use anyhow::Result;
use serde::Deserialize;

create_tauri_wrapper!(magma_decrypt, MagmaDecrypt);

impl Operation<'_, DeserializeMeDaddy> for MagmaDecrypt {
    fn do_black_magic(&self, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let (
            input,
            Params {
                key,
                iv,
                mode,
                input_format,
                ..
            },
        ) = (request.input, request.params);

        let mut akrypt = Akrypt::new(AkryptFunction::Kuznechik);

        akrypt.set_input(&input, input_format)?
            .set_iv(iv.as_bytes())?
            .set_key(key.as_bytes())?
            .set_mode(mode);

        akrypt.decrypt()
    }
}

#[derive(Deserialize)]
struct Params {
    key: String,
    iv: String,
    mode: Mode,
    input_format: InputFormat,
    _output_format: Option<OutputFormat>,
}

create_me_daddy!();

pub struct MagmaDecrypt;
