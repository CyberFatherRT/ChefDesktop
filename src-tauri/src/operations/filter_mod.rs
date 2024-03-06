use crate::{utils::SupportedDelimiter, Operation};
use anyhow::Result;
use serde::Deserialize;

impl Operation<'_, DeserializeMeDaddy> for Filter {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let DeserializeMeDaddy {
            delimiter,
            regex,
            invert_condition,
        } = request;

        let regex = regex::Regex::new(&regex)?;

        let output: String = input
            .split(&delimiter.to_string())
            .filter(|x| regex.is_match(x) != invert_condition)
            .collect();

        Ok(output)
    }
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    delimiter: SupportedDelimiter,
    regex: String,
    invert_condition: bool,
}

pub struct Filter;
