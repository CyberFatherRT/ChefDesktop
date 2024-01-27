use crate::{
    traits::CharTrait,
    utils::{
        get_alphabet, get_char_by_index, get_index_by_char, modulus, validate_lang,
        SupportedLanguages,
    },
};
use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use num::Integer;

pub fn affine_cipher_encode(
    input: &str,
    lang: SupportedLanguages,
    a: i16,
    b: i16,
) -> Result<String> {
    if !validate_lang(input, &lang) {
        bail!("Wrong language.");
    };

    let (alp_lower, alp_upper, _, _, alp_length, _) = get_alphabet(&lang);
    if a.gcd(&(alp_length as i16)) != 1 {
        return Err(anyhow!(
            "The value of `a` must be coprime to alphabet length({}).",
            alp_length
        ));
    }

    let mut output = String::with_capacity(alp_length as usize);

    for c in input.chars() {
        if !c.is_alphabetic() {
            output.push(c);
            continue;
        }

        let x = match c.is_lowercase() {
            true => get_index_by_char(alp_lower, c),
            false => get_index_by_char(alp_upper, c),
        } as i16;

        let x = modulus(a * x + b, alp_length as i16);

        output.push(match c.is_lowercase() {
            true => get_char_by_index(alp_lower, x),
            false => get_char_by_index(alp_upper, x).to_upper_case(),
        });
    }

    Ok(output)
}

pub fn gen_polybius_square(alphabet: &SupportedLanguages, keyword: &str) -> Vec<char> {
    let (_, _, _, alpha, _, _) = get_alphabet(alphabet);
    let pol_array = format!("{}{}", keyword, alpha)
        .chars()
        .unique()
        .collect::<Vec<_>>();

    let size = match alphabet {
        SupportedLanguages::EN => 5,
        SupportedLanguages::RU | SupportedLanguages::RU_WITH_YO => 6,
    };
    let mut polybius: Vec<char> = Vec::new();

    for i in 0..size {
        polybius.extend(pol_array[i * size..i * size + size].iter());
    }

    polybius
}
