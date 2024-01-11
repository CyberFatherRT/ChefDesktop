use crate::{create_me_daddy, utils::SupportedDelimiter, Operation};
use serde::Deserialize;

impl Operation<'_, DeserializeMeDaddy, String> for Filter {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (
            input,
            Params {
                delimiter,
                regex,
                invert_condition,
            },
        ) = (request.input, request.params);

        let regex = regex::Regex::new(&regex).map_err(|err| err.to_string())?;

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
