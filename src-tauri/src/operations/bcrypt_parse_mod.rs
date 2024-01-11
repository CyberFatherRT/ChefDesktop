use crate::{create_info_struct, Operation, DOCS_URL};
use bcrypt::BcryptError;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

impl Operation<'_, DeserializeMeDaddy, HashParts> for BcryptParse {
    fn do_black_magic(&self, request: &str) -> Result<HashParts, String> {
        let request = self.validate(request)?;
        let hash = request.hash;

        let mut parts = HashParts {
            cost: 0,
            salt: "".to_string(),
            hash: "".to_string(),
        };

        let raw_parts: Vec<_> = hash.split('$').filter(|s| !s.is_empty()).collect();

        if raw_parts.len() != 3 {
            return Err(BcryptError::InvalidHash(hash.to_string()).to_string());
        }

        if raw_parts[0] != "2y"
            && raw_parts[0] != "2b"
            && raw_parts[0] != "2a"
            && raw_parts[0] != "2x"
        {
            return Err(BcryptError::InvalidPrefix(raw_parts[0].to_string()).to_string());
        }

        if let Ok(c) = raw_parts[1].parse::<u32>() {
            parts.cost = c;
        } else {
            return Err(BcryptError::InvalidCost(raw_parts[1].to_string()).to_string());
        }

        if raw_parts[2].len() == 53 && raw_parts[2].is_char_boundary(22) {
            parts.salt = raw_parts[2][..22].chars().collect();
            parts.hash = raw_parts[2][22..].chars().collect();
        } else {
            return Err(BcryptError::InvalidHash(hash.to_string()).to_string());
        }

        Ok(parts)
    }
}

#[derive(Deserialize)]
pub struct DeserializeMeDaddy {
    hash: String,
}

#[derive(Serialize, Validate)]
pub struct HashParts {
    #[validate(maximum = 31)]
    #[validate(minimum = 4)]
    cost: u32,
    salt: String,
    hash: String,
}

/// Parses a bcrypt hash to determine the number of rounds used, the salt, and the password hash.
/// <br><br/>
/// For more information about cipher/hash_function go [here](https://wikipedia.org/wiki/Bcrypt)
/// For more information about this function go [here](DOCS_URL)
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/BcryptParse with your data using json payload with this structure
/// ``` json
/// {
///     "hash": string,
/// }
/// ```
/// <br/><br/>
///
/// ## Server response have two possible formats
///
/// ### &nbsp;&nbsp;&nbsp;&nbsp; Ok variant
/// ``` json
/// {
///     "Ok": {
///         "cost": i31,
///         "salt": string,
///         "hash": string
///     }
/// }
/// ```
/// #### where
///     - i31 is signed digit between 4 and 31
/// ### &nbsp;&nbsp;&nbsp;&nbsp; Error variant
/// ``` json
/// { "Err": `error message` }
/// ```
/// # Examples
/// ## №1
/// ``` http
/// POST /api/BcryptParse
///
/// {
///     "hash": "$2x$04$hC6BHE9hPEQZExczLDTxBOgq48yNMI7HC5bmE0HiP/iGxtMpwryh6"
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": {
///     "cost": 4,
///     "salt": "hC6BHE9hPEQZExczLDTxBO",
///     "hash": "gq48yNMI7HC5bmE0HiP/iGxtMpwryh6"
///    }
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/BcryptParse
///
/// {
///     "hash": "$2b$12$mLDUe/nTaPt06W2ai4YrVeCiPK7/L1Dhj7FipakSCnKIDsgqbvPgm"
/// }
/// ```
/// ```http
/// {
///   "Ok": {
///     "cost": 12,
///     "salt": "mLDUe/nTaPt06W2ai4YrVe",
///     "hash": "CiPK7/L1Dhj7FipakSCnKIDsgqbvPgm"
///    }
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/BcryptParse
///
/// {
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `hash`."
/// }
/// ```
pub struct BcryptParse;

const NAME: &str = "BcryptParse";
const DESCRIPTION_EN: &str =
    "Parses a bcrypt hash to determine the number of rounds used, the salt, and the password hash.";
const DESCRIPTION_RU: &str =
    "Анализирует хэш bcrypt для определения количества использованных раундов, соли и хэша пароля.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Bcrypt");

create_info_struct!(
    BcryptParseInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
