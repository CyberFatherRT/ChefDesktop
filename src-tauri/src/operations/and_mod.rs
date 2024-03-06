use crate::{
    create_info_struct,
    libs::bitwise_op::{and as and_fun, bit_op},
    run_op,
    utils::{convert_to_byte_array, SupportedFormats},
    Operation, DOCS_URL,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

run_op!(run_and, AND);

impl Operation<'_, DeserializeMeDaddy> for AND {
    fn do_black_magic(&self, input: &str, request: &str) -> Result<String> {
        let request = self.validate(request)?;
        let DeserializeMeDaddy { key, key_format } = request;

        let key = convert_to_byte_array(&key, &key_format)?;

        let output = String::from_utf8(bit_op(&key, input.as_bytes(), and_fun))?;

        Ok(output)
    }
}

#[derive(Deserialize)]
struct DeserializeMeDaddy {
    key: String,
    key_format: SupportedFormats,
}

/// AND the input with the given key.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Bitwise_operation#Bitwise_operators)
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/AND with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         key: String,
///         key_format: SupportedFormats
///     }
/// }
/// ```
/// #### where
///     SupportedFormats is enum of 'binary', 'utf8', 'hex', 'base64', 'latin1'
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
/// POST /api/AND
///
/// {
///     "input": "hello",
///     "params": {
///         key: "key",
///         key_format: "utf8"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "hehhe"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/AND
///
/// {
///     "input": "hexadecimal",
///     "params": {
///         key: "deadbeef",
///         key_format: "hex"
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "H%8aD%"iL!,"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/AND
///
/// {
///     "input": "error",
///     "params": {
///         key: "no formet",
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `key_format`"
/// }
/// ```
pub struct AND;

const NAME: &str = "AND";
const DESCRIPTION_EN: &str = "AND the input with the given key.";
const DESCRIPTION_RU: &str = "Применяет операцию ADD по модулю 255.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Bitwise_operation#AND");

create_info_struct!(
    ANDInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
