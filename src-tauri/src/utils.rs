use crate::{libs::base64::from_base64, map, regex_check};
use anyhow::{bail, Result};
use lazy_static::lazy_static;
use num::{Integer, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Debug, LowerHex},
};

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SupportedLanguages {
    EN,
    RU,
    #[allow(non_camel_case_types)]
    RU_WITH_YO,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SupportedFormats {
    BINARY,
    UTF8,
    HEX,
    BASE64,
    LATIN1,
}

#[derive(Deserialize, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum SupportedDelimiter {
    Space,
    #[serde(rename = "Line feed")]
    LineFeed,
    CRLF,
    Comma,
    #[serde(rename = "Semi-colon")]
    SemiColon,
    Colon,
}

impl std::fmt::Display for SupportedDelimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub const EN_ALP: (&str, &str, &str, &str, u8, &str) = (
    "abcdefghijklmnopqrstuvwxyz",
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    "abcdefghiklmnopqrstuvwxyz",
    "ABCDEFGHIKLMNOPQRSTUVWXYZ",
    26,
    r"^[a-zA-Z]+$",
);
pub const RU_ALP: (&str, &str, &str, &str, u8, &str) = (
    "абвгдежзийклмнопрстуфхцчшщъыьэюя",
    "АБВГДЕЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ",
    "абвгдежзийклмнопрстуфхцчшщъыьэюя,.-=",
    "АБВГДЕЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ,.-=",
    32,
    "^[а-яА-Я]+$",
);
pub const RU_ALP_WITH_YO: (&str, &str, &str, &str, u8, &str) = (
    "абвгдеёжзийклмнопрстуфхцчшщъыьэюя",
    "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ",
    "абвгдеёжзийклмнопрстуфхцчшщъыьэюя,.-",
    "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ,.-",
    33,
    r"^[а-яА-ЯёЁ]+$",
);

lazy_static! {
    static ref CHAR_REPR: HashMap<&'static str, &'static str> = map!("Space" => " ",
        "Percent" => "%",
        "Comma" => ",",
        "Semi-colon" => ";",
        "Colon" => ":",
        "Tab" => "\t",
        "Line feed" => "\n",
        "CRLF" => "\r\n",
        "Forward slash" => "/",
        "Backslash" => "\\",
        "0x" => "0x",
        "\\x" => "\\x",
        "Nothing (separate chars)" => "",
        "None" => ""
    );
}

pub fn convert_to_byte_array(string: &str, convert_type: &SupportedFormats) -> Result<Vec<u8>> {
    match convert_type {
        SupportedFormats::BINARY => from_binary(string, None, None),
        SupportedFormats::HEX => from_hex(string),
        SupportedFormats::BASE64 => from_base64(string),
        SupportedFormats::UTF8 => Ok(string.as_bytes().to_vec()),
        SupportedFormats::LATIN1 => Ok(Vec::new()),
    }
}

pub fn from_binary(data: &str, delim: Option<&str>, byte_len: Option<usize>) -> Result<Vec<u8>> {
    if byte_len.unwrap_or(8) < 1 {
        bail!("Byte length must be a positive integer");
    };

    let delim = char_repr(delim.unwrap_or("Space"));
    let data = data.replace(delim, " ");

    let mut output: Vec<u8> = Vec::new();
    for i in data.split_whitespace() {
        match u8::from_str_radix(i, 2) {
            Ok(data) => output.push(data),
            Err(e) => bail!(e),
        }
    }

    Ok(output)
}

pub fn to_hex(data: &[u8]) -> String {
    data.iter()
        .fold(String::new(), |out, x| format!("{out}{x:02x}"))
}

pub fn from_hex(data: &str) -> Result<Vec<u8>> {
    Ok(hex::decode(format!(
        "{data:0>fill$}",
        fill = data.len() + data.len() % 2
    ))?)
}

pub fn validate_lang(text: &str, lang: &SupportedLanguages) -> bool {
    let re = match lang {
        SupportedLanguages::EN => r"^[a-zA-Z\p{P}\s\d]+$",
        SupportedLanguages::RU => r"^[а-яА-Я\p{P}\s\d]+$",
        SupportedLanguages::RU_WITH_YO => r"^[а-яА-ЯёЁ\p{P}\s\d]+$",
    };
    regex_check!(re => text)
}

pub fn get_alphabet(
    lang: &SupportedLanguages,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    u8,
    &'static str,
) {
    match lang {
        SupportedLanguages::EN => EN_ALP,
        SupportedLanguages::RU => RU_ALP,
        SupportedLanguages::RU_WITH_YO => RU_ALP_WITH_YO,
    }
}

pub fn get_char_by_index<T: Integer + ToPrimitive>(text: &str, index: T) -> char {
    text.chars().nth(index.to_usize().unwrap()).unwrap()
}

pub fn get_index_by_char(text: &str, ch: char) -> usize {
    text.chars().position(|c| c == ch).unwrap()
}

pub fn char_repr(token: &str) -> &str {
    CHAR_REPR.get(token).unwrap_or(&" ")
}

pub fn update_step<T: Integer + Copy>(a: &mut T, old_a: &mut T, quotient: T) {
    let temp = *a;
    *a = *old_a - quotient * temp;
    *old_a = temp;
}

pub fn extended_gcd<T: Integer + Copy>(a: T, b: T) -> (T, T, T) {
    let (mut old_r, mut rem) = (a, b);
    let (mut old_s, mut coefficient_s) = (T::one(), T::zero());
    let (mut old_t, mut coefficient_t) = (T::zero(), T::one());

    while rem != T::zero() {
        let quotient = old_r / rem;

        update_step(&mut rem, &mut old_r, quotient);
        update_step(&mut coefficient_s, &mut old_s, quotient);
        update_step(&mut coefficient_t, &mut old_t, quotient);
    }

    (old_r, old_s, old_t)
}

pub fn mod_inv<T: Integer + Copy>(a: T, module: T) -> T {
    let (_, x, _) = extended_gcd(a, module);

    if x < T::zero() {
        x + module
    } else {
        x
    }
}

pub fn modulus<T: Integer + Copy>(x: T, y: T) -> T {
    ((x % y) + y) % y
}

pub fn add(a: i16, b: i16) -> i16 {
    a + b
}

pub fn sub(a: i16, b: i16) -> i16 {
    a - b
}

pub fn hex<T: Debug + LowerHex>(c: T) -> String {
    format!("{:08x}", c)
}
