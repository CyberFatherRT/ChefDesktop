use crate::{
    create_info_struct, create_me_daddy,
    libs::bitwise_op::{add, bit_op},
    utils::{convert_to_byte_array, SupportedFormats},
    Operation, DOCS_URL,
};
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy, String> for ADD {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (input, Params { key, key_format }) = (request.input, request.params);

        let key = convert_to_byte_array(&key, &key_format)?;

        let output = String::from_utf8(bit_op(&key, input.as_bytes(), add))
            .map_err(|err| err.to_string())?;

        Ok(output)
    }
}

#[derive(Deserialize)]
struct Params {
    key: String,
    key_format: SupportedFormats,
}

create_me_daddy!();

/// ADD the input with the given key, MOD 255
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
pub struct ADD;

const NAME: &str = "ADD";
const DESCRIPTION_EN: &str = "ADD the input with the given key, MOD 255";
const DESCRIPTION_RU: &str = "Применяет операцию ADD по модулю 255.";

const INFO_URL: Option<&str> =
    Some("https://wikipedia.org/wiki/Bitwise_operation#Bitwise_operators");

create_info_struct!(
    ADDInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
