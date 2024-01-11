use regex::Regex;
use std::collections::HashMap;

use crate::{
    traits::CharTrait,
    utils::{get_alphabet, get_char_by_index, modulus, validate_lang, SupportedLanguages},
};

pub trait VigenereCipher {
    fn cipher<F>(lang: SupportedLanguages, key: &str, input: &str, f: F) -> Result<String, String>
    where
        F: Fn(i16, i16) -> i16,
    {
        <Self as VigenereCipher>::validate_language(&lang, key, input)?;

        let (alp, _, _, _, _, reg) = get_alphabet(&lang);

        let map: HashMap<char, usize> =
            HashMap::from_iter(alp.chars().enumerate().map(|(idx, elem)| (elem, idx)));

        let key = key.to_lowercase();
        let rg = Regex::new(reg).unwrap();
        let mut index = 0usize;
        let mut cipher_text = String::new();

        let key_len = key.chars().count();
        let alp_len = alp.chars().count() as i16;

        for c in input.chars() {
            if !rg.is_match(&c.to_string()) {
                cipher_text.push(c);
                continue;
            }

            let key_idx = map
                .get(&get_char_by_index(&key, index % key_len))
                .unwrap()
                .to_owned() as i16;

            let text_idx = match c.is_lowercase() {
                true => map.get(&c).unwrap(),
                false => map.get(&c.to_lower_case()).unwrap(),
            }
            .to_owned() as i16;

            let idx = f(text_idx, key_idx);

            let plain_char = get_char_by_index(alp, modulus(idx, alp_len));
            cipher_text.push(match c.is_lowercase() {
                true => plain_char,
                false => plain_char.to_upper_case(),
            });

            index += 1;
        }
        Ok(cipher_text)
    }

    fn validate_language(lang: &SupportedLanguages, key: &str, input: &str) -> Result<(), String> {
        if input.is_empty() {
            return Err("Input is empty".to_string());
        };

        if !validate_lang(key, lang) {
            return Err("Invalid key".to_string());
        };

        Ok(())
    }
}
