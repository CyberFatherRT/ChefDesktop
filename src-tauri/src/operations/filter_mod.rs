use crate::{
    create_me_daddy, create_tauri_wrapper, run_operations, utils::SupportedDelimiter, Operation,
};
use anyhow::Result;
use serde::Deserialize;

create_tauri_wrapper!(filter, Filter);

impl Operation<'_, DeserializeMeDaddy> for Filter {
    fn do_black_magic(&self, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let (
            input,
            Params {
                delimiter,
                regex,
                invert_condition,
            },
        ) = (request.input, request.params);

        let regex = regex::Regex::new(&regex)?;

        let output: String = input
            .split(&delimiter.to_string())
            .filter(|x| regex.is_match(x) != invert_condition)
            .collect();

        Ok(output)
    }
}

#[derive(Deserialize)]
struct Params {
    delimiter: SupportedDelimiter,
    regex: String,
    invert_condition: bool,
}

create_me_daddy!();

pub struct Filter;
