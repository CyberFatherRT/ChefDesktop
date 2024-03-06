use crate::{create_info_struct, run_op, Operation, DOCS_URL};
use anyhow::Result;
use serde::Serialize;

run_op!(run_addlinenumbers, AddLineNumbers);

impl Operation<'_, ()> for AddLineNumbers {
    fn do_black_magic(&self, input: &str, _request: &str) -> Result<String> {
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
