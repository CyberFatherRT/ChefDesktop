use std::collections::BTreeMap;

use serde::Deserialize;

use crate::utils::{get_alphabet, SupportedLanguages};

pub struct BaconCipher {
    en_map: BTreeMap<char, String>,
    de_map: BTreeMap<String, String>,
}

impl BaconCipher {
    pub fn new(
        item_a: char,
        item_b: char,
        translation: SupportedBaconTranslation,
        alp: SupportedBaconAlphabet,
        lang: SupportedLanguages,
    ) -> Self {
        let (alphabet, _, _, _, length, _) = get_alphabet(&lang);

        let en_map = alphabet
            .chars()
            .map(|c| {
                Self::char_by_alphabet(c, alphabet, &translation, &alp, &length, &item_a, &item_b)
            })
            .collect();

        let de_map = alphabet
            .chars()
            .map(|c| {
                let x = Self::char_by_alphabet(
                    c,
                    alphabet,
                    &translation,
                    &alp,
                    &length,
                    &item_a,
                    &item_b,
                );
                (x.1, x.0.to_string())
            })
            .collect();

        BaconCipher { en_map, de_map }
    }

    pub fn encode(&self, elem: &str) -> Vec<String> {
        elem.to_lowercase()
            .chars()
            .map(|x| match self.en_map.get(&x) {
                None => x.to_string(),
                Some(data) => data.to_owned(),
            })
            .collect()
    }

    pub fn decode(&self, elem: &str) -> Vec<String> {
        elem.split_whitespace()
            .map(|x| match self.de_map.get(x) {
                None => x.to_owned(),
                Some(data) => data.to_owned(),
            })
            .collect()
    }
}

impl BaconCipher {
    fn char_by_alphabet(
        c: char,
        alphabet: &str,
        translation: &SupportedBaconTranslation,
        alp: &SupportedBaconAlphabet,
        length: &u8,
        a: &char,
        b: &char,
    ) -> (char, String) {
        let костыль = match alp {
            SupportedBaconAlphabet::Standard => {
                if ('a'..'j').contains(&c) {
                    0
                } else if ('j'..'v').contains(&c) {
                    1
                } else {
                    2
                }
            }
            SupportedBaconAlphabet::Complete => 0,
        };

        let string = match (*length as f64).sqrt().round() as i8 {
            5 => format!("{:05b}", alphabet.find(c).unwrap() - костыль),
            6 => format!("{:06b}", alphabet.find(c).unwrap()),
            _ => unreachable!(),
        }
        .chars()
        .map(|x| match translation {
            SupportedBaconTranslation::ZeroOne => x,
            SupportedBaconTranslation::AB => match x {
                '0' => *a,
                '1' => *b,
                _ => unreachable!(),
            },
        })
        .collect();

        (c, string)
    }
}

impl Default for BaconCipher {
    fn default() -> Self {
        BaconCipher::new(
            'A',
            'B',
            SupportedBaconTranslation::AB,
            SupportedBaconAlphabet::Standard,
            SupportedLanguages::EN,
        )
    }
}

#[derive(Deserialize)]
pub enum SupportedBaconTranslation {
    #[serde(rename = "0/1")]
    ZeroOne,
    #[serde(rename = "A/B")]
    AB,
}

#[derive(Deserialize)]
pub enum SupportedBaconAlphabet {
    #[serde(alias = "Standard (I=J and V=U)")]
    Standard,
    Complete,
}
