use serde::{Deserialize, Serialize};

pub fn bit_op<F>(input: &[u8], key: &[u8], func: F) -> Vec<u8>
where
    F: Fn(u8, u8) -> u8,
{
    input
        .iter()
        .enumerate()
        .map(|(i, &e)| func(e, key[i % key.len()]))
        .collect()
}

pub fn xor(operand: u8, key: u8) -> u8 {
    operand ^ key
}

pub fn not(operand: u8, _: u8) -> u8 {
    (!operand as i16 & 0xffi16) as u8
}

pub fn and(operand: u8, key: u8) -> u8 {
    operand & key
}

pub fn or(operand: u8, key: u8) -> u8 {
    operand | key
}

pub fn add(operand: u8, key: u8) -> u8 {
    ((operand as i16 + key as i16) % 255) as u8
}

pub fn sub(operand: u8, key: u8) -> u8 {
    let result = operand as i16 - key as i16;
    if result < 0 {
        (result + 256) as u8
    } else {
        result as u8
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BitwiseOpDelimiters {
    Hex,
    Decimal,
    Binary,
    Base64,
    Utf8,
    Latin1,
}
