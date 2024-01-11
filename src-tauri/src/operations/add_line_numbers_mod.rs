use crate::{create_info_struct, Operation, DOCS_URL};
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy, String> for AddLineNumbers {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let input = request.input;
        let output = input
            .split('\n')
            .enumerate()
            .fold(String::new(), |acc, (i, x)| {
                let index = " "
                    .repeat(input.len() - (i + 1).ilog10() as usize - 1)
                    .to_string()
                    + &(i + 1).to_string();
                acc + &format!("{index} {x}")
            });

        Ok(output)
    }
}

#[derive(Deserialize)]
pub struct DeserializeMeDaddy {
    input: String,
}

pub struct AddLineNumbers;

const NAME: &str = "Add line numbers";
const DESCRIPTION_EN: &str = "Adds line numbers to the output.";
const DESCRIPTION_RU: &str = "Добавляет номера строк в выходные данные.";

const INFO_URL: Option<&str> = None;

create_info_struct!(
    AddLineNumbersInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
