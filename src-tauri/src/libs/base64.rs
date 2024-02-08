use anyhow::{anyhow, bail, Result};
use base64::Engine;
use itertools::Itertools;

use crate::{
    traits::StringTrait,
    utils::{
        expand_alphabet_range, get_char_by_index, str_to_array_buffer_by_alphabet,
        DataRepresentation, DataRepresentationInput,
    },
};

pub fn bytes_to_base64(data: &[u8]) -> String {
    base64::prelude::BASE64_STANDARD.encode(data)
}

pub fn from_base64(
    mut data: String,
    mut alphabet: &str,
    return_type: DataRepresentationInput,
    remove_non_alphabetic_chars: bool,
    strict_mode: bool,
) -> Result<DataRepresentation> {
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
            bail!("Input string isn't correspond to used base64 alphabet.");
        }
    }

    let alphabet = expand_alphabet_range(alphabet).iter().collect::<String>();
    let alphabet_length = alphabet.chars().count();

    if alphabet_length != 64 && alphabet_length != 65 {
        bail!("Invalid base64 alphabet length.");
    }

    if remove_non_alphabetic_chars {
        data = data.replace_by_alphabet(&alphabet)
    }

    if strict_mode {
        if data.len() % 4 == 1 {
            return Err(anyhow!(
                "Invalid Base64 input length ({}) cannot be 4n+1, even without padding chars.",
                data.len()
            ));
        }

        if alphabet_length == 65 {
            let pad = get_char_by_index(&alphabet, 64);
            let pad_pos = data.find(pad);

            if let Some(pad_pos) = pad_pos {
                if pad_pos < data.len() - 2 || get_char_by_index(&data, data.len() - 1) != pad {
                    bail!("Base64 padding character ({pad}) not used in the correct place.");
                }

                if data.len() % 4 != 0 {
                    bail!("Base64 not padded to a multiple of 4.");
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
