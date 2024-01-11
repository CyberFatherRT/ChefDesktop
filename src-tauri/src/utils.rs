use crate::{libs::base64::from_base64, map, regex_check, traits::StringTrait};
use num::{Integer, ToPrimitive};
use serde::Deserialize;
use std::fmt::{Debug, LowerHex};

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SupportedLanguages {
    EN,
    RU,
    #[allow(non_camel_case_types)]
    RU_WITH_YO,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SupportedFormats {
    BINARY,
    UTF8,
    HEX,
    BASE64,
    LATIN1,
}

#[derive(Eq, PartialEq, Debug)]
pub enum DataRepresentation {
    String(String),
    ByteArray(Vec<u8>),
}

#[derive(Eq, PartialEq, Debug)]
pub enum DataRepresentationInput {
    String,
    ByteArray,
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
pub const NUM: (&str, &str) = ("0123456789", r"^\+?(0|[1-9]\d*)$");

pub fn expand_alphabet_range(alphabet: &str) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    let alphabet_length = alphabet.chars().count();
    let mut i = 0;

    while i < alphabet_length {
        let by_index = get_char_by_index(alphabet, i);
        if (i < alphabet_length - 2)
            && (get_char_by_index(alphabet, i + 1) == '-')
            && (by_index != '\\')
        {
            let (start, end) = (ord(by_index), ord(get_char_by_index(alphabet, i + 2)));

            for j in start..=end {
                result.push(chr(j));
            }
            i += 2;
        } else if (i < alphabet_length - 2)
            && (by_index == '\\')
            && (get_char_by_index(alphabet, i + 1) == '-')
        {
            result.push('-');
            i += 1;
        } else {
            result.push(by_index);
        }
        i += 1;
    }

    result
}

pub fn str_to_array_buffer(string: &str) -> Vec<u32> {
    if string.is_empty() {
        return Vec::new();
    }

    let string_length = string.chars().count();
    let mut result: Vec<u32> = vec![0; string_length];

    for (idx, elem) in result.iter_mut().enumerate() {
        *elem = ord(get_char_by_index(string, idx));
    }

    result
}

pub fn str_to_array_buffer_by_alphabet(string: &str, alphabet: &str) -> Vec<usize> {
    if string.is_empty() {
        return Vec::new();
    }

    let string_length = string.chars().count();
    let mut result: Vec<usize> = vec![0; string_length];
    for (idx, c) in string.chars().enumerate() {
        result[idx] = get_index_by_char(alphabet, c);
    }
    result
}

pub fn byte_array_to_string(byte_array: Vec<u8>) -> Result<String, String> {
    String::from_utf8(byte_array).map_err(|err| err.to_string().capitalize() + ".")
}

pub fn convert_to_byte_array(
    string: &str,
    convert_type: &SupportedFormats,
) -> Result<Vec<u8>, String> {
    match convert_type {
        SupportedFormats::BINARY => from_binary(string, None, None).map_err(|err| err.to_string()),
        SupportedFormats::HEX => from_hex(string, None, None).map_err(|err| err.to_string()),
        SupportedFormats::BASE64 => match from_base64(
            string.to_string(),
            "",
            DataRepresentationInput::ByteArray,
            true,
            false,
        )
        .map_err(|err| err.to_string())
        {
            Ok(data) => {
                let DataRepresentation::ByteArray(data) = data else {
                    unreachable!()
                };
                Ok(data)
            }
            Err(e) => Err(e),
        },
        SupportedFormats::UTF8 => Ok(string.as_bytes().to_vec()),
        SupportedFormats::LATIN1 => Ok(Vec::new()),
    }
}

pub fn from_binary(
    data: &str,
    delim: Option<&str>,
    byte_len: Option<usize>,
) -> Result<Vec<u8>, String> {
    if byte_len.unwrap_or(8) < 1 {
        return Err("Byte length must be a positive integer".to_string());
    };

    let delim = char_repr(delim.unwrap_or("Space"));
    let data = data.replace(delim, " ");

    let mut output: Vec<u8> = Vec::new();
    for i in data.split_whitespace() {
        match u8::from_str_radix(i, 2) {
            Ok(data) => output.push(data),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(output)
}

pub fn to_hex(data: &[u8]) -> String {
    data.iter()
        .fold(String::new(), |out, x| format!("{out}{x:02x}"))
}

pub fn from_hex(
    data: &str,
    delim: Option<&str>,
    byte_len: Option<usize>,
) -> Result<Vec<u8>, String> {
    if byte_len.unwrap_or(8) < 1 {
        return Err("Byte length must be a positive integer".to_string());
    }

    let mut output: Vec<u8> = Vec::new();

    let delim = char_repr(delim.unwrap_or("Space"));

    for i in data.split(&delim) {
        match u8::from_str_radix(i, 16) {
            Ok(data) => output.push(data),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(output)
}

pub fn from_decimal(data: &str, delim: Option<&str>) -> Result<Vec<usize>, String> {
    let mut output = Vec::new();
    for i in data.split(char_repr(delim.unwrap_or("Space"))) {
        match i.parse::<usize>() {
            Ok(data) => output.push(data),
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(output)
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

#[inline]
pub fn get_char_by_index<T: Integer + ToPrimitive>(text: &str, index: T) -> char {
    text.chars().nth(index.to_usize().unwrap()).unwrap()
}

pub fn get_index_by_char(text: &str, ch: char) -> usize {
    text.chars().position(|c| c == ch).unwrap()
}

pub fn char_repr(token: &str) -> &str {
    map!(
        "Space" => " ",
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
        "None" => "",
    )
    .get(token)
    .unwrap_or(&" ")
}

pub fn chr<T: ToPrimitive>(code: T) -> char {
    char::from_u32(code.to_u32().unwrap()).unwrap()
}

pub fn ord(chr: char) -> u32 {
    chr as u32
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
