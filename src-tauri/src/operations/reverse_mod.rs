use crate::Operation;
use anyhow::Result;

impl Operation<'_, ()> for ReverseString {
    fn do_black_magic(&self, input: &str, _request: &str) -> Result<String> {
        Ok(input.chars().rev().collect())
    }
}

pub struct ReverseString;
