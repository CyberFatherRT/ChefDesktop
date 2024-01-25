use num::{BigInt, Num};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

use crate::{
    create_info_struct, create_me_daddy, create_tauri_wrapper, run_operations, Operation, DOCS_URL,
};

create_tauri_wrapper!(to_base, ToBase, String, String);

impl Operation<'_, DeserializeMeDaddy, String> for ToBase {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (input, radix) = (request.input, request.params.radix);

        #[allow(non_snake_case)]
        let big_D_number = BigInt::from_str_radix(&input, 10)
            .map_err(|_| "Invalid symbols found in string".to_string())?;
        Ok(big_D_number.to_str_radix(radix))
    }
}

#[derive(Deserialize, Validate)]
struct Params {
    #[validate(maximum = 36)]
    #[validate(minimum = 2)]
    radix: u32,
}

create_me_daddy!();

/// Converts a number from decimal to a given numerical base.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Radix).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/ToBase with your data using json payload with this structure.
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
/// POST /api/ToBase
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
///   "Ok": "83"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/ToBase
///
/// {
///     "input": "3735928559",
///     "params": {
///         "radix": 16
///     }
/// }
/// ``
/// ```
/// ```http
/// {
///   "Ok": "deadbeef"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/ToBase
/// content_type: application/json; charset=utf-8
///
/// {
///     "input": "69",
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

pub struct ToBase;

const NAME: &str = "ToBase";
const DESCRIPTION_EN: &str = "Converts a number from decimal to a given numerical base.";
const DESCRIPTION_RU: &str = "Преобразует число из десятичного в заданное числовое основание.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Radix");

create_info_struct!(
    ToBaseInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
