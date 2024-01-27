use crate::{create_tauri_wrapper, run_operations, Operation};
use anyhow::Result;
use serde::Deserialize;

create_tauri_wrapper!(reverse, ReverseString);

impl Operation<'_, DeserializeMeDaddy> for ReverseString {
    fn do_black_magic(&self, request: &str) -> Result<String> {
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
