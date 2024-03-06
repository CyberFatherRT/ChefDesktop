use crate::{create_info_struct, Operation, DOCS_URL};

use anyhow::{bail, Result};
use num::{BigInt, Num};
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy> for FromBase {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let radix = request.radix;

        #[allow(non_snake_case)]
        let big_D_number = BigInt::from_str_radix(&input, radix)?;
        Ok(big_D_number.to_string())
    }

    fn validate(&self, request: &'_ str) -> Result<DeserializeMeDaddy> {
        let request = self.deserialize(request)?;

        if !(2..=36).contains(&request.radix) {
            bail!("ERROR: Param `rounds` must be between 2 and 36")
        }

        Ok(request)
    }
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    radix: u32,
}

/// Converts a number to decimal from a given numerical base.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Radix).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/FromBase with your data using json payload with this structure.
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "radix": u36,
///     }
/// }
/// ```
/// #### where
///     - u36 is digit between 2 and 36 inclusive
/// <br/><br/>
///
/// ### Server response have two possible formats
///
/// #### &nbsp;&nbsp;&nbsp;&nbsp; Ok variant
/// ``` json
/// { "Ok": `some answer` }
/// ```
/// #### &nbsp;&nbsp;&nbsp;&nbsp; Error variant
/// ``` json
/// { "Err": `error message` }
/// ```
/// # Examples
/// ## №1
/// ``` http
/// POST /api/FromBase
///
/// {
///     "input": "123",
///     "params": {
///         "radix": 15
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "258"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/FromBase
///
/// {
///     "input": "6ba754a0cd1c90784e4c0ade35dbe8dfcceaf520e77c51a5b21ce5c2",
///     "params": {
///         "radix": 33
///     }
/// }
/// ``
/// ```
/// ```http
/// {
///   "Ok": "2091904736137844619429815059353840143519387965190608554254592830964315299383632619562"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/FromBase
/// content_type: application/json; charset=utf-8
///
/// {
///     "input": "deadbeef",
///     "params": {
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `radix`."
/// }
/// ```
pub struct FromBase;

const NAME: &str = "FromBase";
const DESCRIPTION_EN: &str = "Converts a number to decimal from a given numerical base.";
const DESCRIPTION_RU: &str = "Преобразует число в десятичное по заданной системе счисления.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Radix");

create_info_struct!(
    FromBaseInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
