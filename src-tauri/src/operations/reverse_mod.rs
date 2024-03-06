use crate::{run_op, Operation};
use anyhow::Result;

run_op!(run_reversestring, ReverseString);

impl Operation<'_, ()> for ReverseString {
    fn do_black_magic(&self, input: &str, _request: &str) -> Result<String> {
        Ok(input.chars().rev().collect())
    }
}

pub struct ReverseString;
