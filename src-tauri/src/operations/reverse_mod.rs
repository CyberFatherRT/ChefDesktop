use crate::Operation;
use serde::Deserialize;

impl Operation<'_, DeserializeMeDaddy, String> for ReverseString {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let input = request.input;
        Ok(input.chars().rev().collect())
    }
}

#[derive(Deserialize)]
pub struct DeserializeMeDaddy {
    input: String,
}

pub struct ReverseString;
