use crate::{
    traits::StringTrait,
    utils::{
        expand_alphabet_range, get_char_by_index, str_to_array_buffer_by_alphabet,
        DataRepresentation, DataRepresentationInput,
    },
};
use itertools::Itertools;

pub fn to_base64(data: &[u8], alphabet: Option<String>) -> Result<String, String> {
    if data.is_empty() {
        return Ok(String::new());
    }

    let alphabet = alphabet.unwrap_or("A-Za-z0-9+/=".to_string());

    let alphabet = String::from_iter(expand_alphabet_range(&alphabet));

    let alphabet_length = alphabet.chars().count();

    if alphabet_length != 64 && alphabet_length != 65 {
        return Err(format!(
            "Invalid base64 alphabet length. ({alphabet_length}):\n{alphabet}"
        ));
    }

    let mut output = String::new();
    let mut padding = 0;

    data.iter()
        .fold(String::new(), |acc, x| acc + &format!("{:08b}", x))
        .chars()
        .chunks(6)
        .into_iter()
        .map(|x| {
            let sextet = x.collect::<String>();
            match sextet.len() {
                6 => u8::from_str_radix(&sextet, 2),
                _ => {
                    padding += 1;
                    u8::from_str_radix(&format!("{:0<6}", sextet), 2)
                }
            }
            .unwrap()
        })
        .for_each(|x| output.push(get_char_by_index(&alphabet, x)));

    output.push_str(&match alphabet_length {
        65 => get_char_by_index(&alphabet, 64).to_string().repeat(padding),
        _ => "".to_string(),
    });

    Ok(output)
}
pub fn from_base64(
    mut data: String,
    mut alphabet: &str,
    return_type: DataRepresentationInput,
    remove_non_alphabetic_chars: bool,
    strict_mode: bool,
) -> Result<DataRepresentation, String> {
    if data.is_empty() {
        return match return_type {
            DataRepresentationInput::String => Ok(DataRepresentation::String(String::new())),
            DataRepresentationInput::ByteArray => Ok(DataRepresentation::ByteArray(Vec::new())),
        };
    }

    if alphabet.is_empty() {
        alphabet = "A-Za-z0-9+/=";
    }

    if !remove_non_alphabetic_chars {
        let regex = regex::Regex::new(&format!("[^{}]", alphabet)).unwrap();
        if regex.is_match(&data) {
            return Err("Input string isn't correspond to used base64 alphabet.".to_string());
        }
    }

    let alphabet = expand_alphabet_range(alphabet).iter().collect::<String>();
    let alphabet_length = alphabet.chars().count();

    if alphabet_length != 64 && alphabet_length != 65 {
        return Err("Invalid base64 alphabet length.".to_string());
    }

    if remove_non_alphabetic_chars {
        data = data.replace_by_alphabet(&alphabet)
    }

    if strict_mode {
        if data.len() % 4 == 1 {
            return Err(format!(
                "Invalid Base64 input length ({}) cannot be 4n+1, even without padding chars.",
                data.len()
            ));
        }

        if alphabet_length == 65 {
            let pad = get_char_by_index(&alphabet, 64);
            let pad_pos = data.find(pad);

            if let Some(pad_pos) = pad_pos {
                if pad_pos < data.len() - 2 || get_char_by_index(&data, data.len() - 1) != pad {
                    return Err(
                        "Base64 padding character ({pad}) not used in the correct place."
                            .to_string(),
                    );
                }

                if data.len() % 4 != 0 {
                    return Err("Base64 not padded to a multiple of 4.".to_string());
                }
            }
        }
    }

    if alphabet_length == 65 {
        data = data
            .trim_end_matches(get_char_by_index(&alphabet, 64))
            .to_string();
    }

    return match return_type {
        DataRepresentationInput::String => {
            let mut output = String::new();
            str_to_array_buffer_by_alphabet(&data, &alphabet)
                .iter()
                .map(|x| format!("{:08b}", x)[2..].to_string())
                .collect::<String>()
                .chars()
                .chunks(8)
                .into_iter()
                .map(|x| u8::from_str_radix(&x.collect::<String>(), 2).unwrap() as char)
                .for_each(|x| output.push(x));

            if output.ends_with('\u{0}') {
                output = output[..output.len() - 1].to_string()
            }

            Ok(DataRepresentation::String(output))
        }
        DataRepresentationInput::ByteArray => {
            let mut output = Vec::new();

            str_to_array_buffer_by_alphabet(&data, &alphabet)
                .iter()
                .map(|x| format!("{:08b}", x)[2..].to_string())
                .collect::<String>()
                .chars()
                .chunks(8)
                .into_iter()
                .map(|x| u8::from_str_radix(&x.collect::<String>(), 2).unwrap())
                .for_each(|x| output.push(x));

            Ok(DataRepresentation::ByteArray(
                output[..output.len() - 1].to_vec(),
            ))
        }
    };
}

#[allow(dead_code)]
pub struct AlphabetOptions {
    name: &'static str,
    value: &'static str,
}

pub const ALPHABET_OPTIONS: &[AlphabetOptions] = &[
    AlphabetOptions {
        name: "Standard (RFC 4648): A-Za-z0-9+/=",
        value: "A-Za-z0-9+/=",
    },
    AlphabetOptions {
        name: "URL safe (RFC 4648 §5): A-Za-z0-9-_",
        value: "A-Za-z0-9-_",
    },
    AlphabetOptions {
        name: "Filename safe: A-Za-z0-9+-=",
        value: "A-Za-z0-9+\\-=",
    },
    AlphabetOptions {
        name: "itoa64: ./0-9A-Za-z=",
        value: "./0-9A-Za-z=",
    },
    AlphabetOptions {
        name: "y64: A-Za-z0-9._-",
        value: "A-Za-z0-9._-",
    },
    AlphabetOptions {
        name: "z64: 0-9a-zA-Z+/=",
        value: "0-9a-zA-Z+/=",
    },
    AlphabetOptions {
        name: "Radix-64 (RFC 4880): 0-9A-Za-z+/=",
        value: "0-9A-Za-z+/=",
    },
    AlphabetOptions {
        name: "Uuencoding: [space]-_",
        value: " -_",
    },
    AlphabetOptions {
        name: "Xxencoding: +-0-9A-Za-z",
        value: "+\\-0-9A-Za-z",
    },
    AlphabetOptions {
        name: "BinHex: !-,-0-689@A-NP-VX-Z[`a-fh-mp-r",
        value: "!-,-0-689@A-NP-VX-Z[`a-fh-mp-r",
    },
    AlphabetOptions {
        name: "ROT13: N-ZA-Mn-za-m0-9+/=",
        value: "N-ZA-Mn-za-m0-9+/=",
    },
    AlphabetOptions {
        name: "UNIX crypt: ./0-9A-Za-z",
        value: "./0-9A-Za-z",
    },
    AlphabetOptions {
        name: "Atom128: /128GhIoPQROSTeUbADfgHijKLM+n0pFWXY456xyzB7=39VaqrstJklmNuZvwcdEC",
        value: "/128GhIoPQROSTeUbADfgHijKLM+n0pFWXY456xyzB7=39VaqrstJklmNuZvwcdEC",
    },
    AlphabetOptions {
        name: "Megan35: 3GHIJKLMNOPQRSTUb=cdefghijklmnopWXYZ/12+406789VaqrstuvwxyzABCDEF5",
        value: "3GHIJKLMNOPQRSTUb=cdefghijklmnopWXYZ/12+406789VaqrstuvwxyzABCDEF5",
    },
    AlphabetOptions {
        name: "Zong22: ZKj9n+yf0wDVX1s/5YbdxSo=ILaUpPBCHg8uvNO4klm6iJGhQ7eFrWczAMEq3RTt2",
        value: "ZKj9n+yf0wDVX1s/5YbdxSo=ILaUpPBCHg8uvNO4klm6iJGhQ7eFrWczAMEq3RTt2",
    },
    AlphabetOptions {
        name: "Hazz15: HNO4klm6ij9n+J2hyf0gzA8uvwDEq3X1Q7ZKeFrWcVTts/MRGYbdxSo=ILaUpPBC5",
        value: "HNO4klm6ij9n+J2hyf0gzA8uvwDEq3X1Q7ZKeFrWcVTts/MRGYbdxSo=ILaUpPBC5",
    },
];
