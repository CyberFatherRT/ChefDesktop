use crate::{create_tauri_wrapper, run_operations, Operation};
use serde::Deserialize;

create_tauri_wrapper!(reverse, ReverseString, String, String);

impl Operation<'_, DeserializeMeDaddy, String> for ReverseString {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let input = request.input;
        Ok(input.chars().rev().collect())
    }
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    input: String,
}

pub struct ReverseString;
