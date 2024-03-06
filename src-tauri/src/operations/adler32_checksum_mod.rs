use crate::{create_info_struct, utils::hex, Operation, DOCS_URL};
use anyhow::Result;
use serde::Serialize;

impl Operation<'_, ()> for Adler32CheckSum {
    fn do_black_magic(&self, input: &str, _request: &str) -> Result<String> {
        let input: Vec<u8> = input.into();

        const MOD_ADLER: isize = 65521;
        let (mut a, mut b): (isize, isize) = (1, 0);

        for i in input.iter() {
            a += *i as isize;
            b += a;
        }

        a %= MOD_ADLER;
        b %= MOD_ADLER;

        Ok(hex(b << 16 | a))
    }
}

pub struct Adler32CheckSum;

const NAME: &str = "Adler32CheckSum";
const DESCRIPTION_EN: &str = "Adler-32 is a checksum algorithm which was invented by Mark Adler in 1995, and is a modification of the Fletcher checksum. Compared to a cyclic redundancy check of the same length, it trades reliability for speed (preferring the latter). Adler-32 is more reliable than Fletcher-16, and slightly less reliable than Fletcher-32.";
const DESCRIPTION_RU: &str = "Adler-32 — алгоритм контрольной суммы, изобретенный Марком Адлером в 1995 году и являющийся модификацией контрольной суммы Флетчера. По сравнению с циклическим избыточным кодом той же продолжительности, он жертвует надежностью ради скорости (предпочитая последнюю). Адлер-32 более надежен, чем Флетчер-16, и немного менее надежен, чем Флетчер-32.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Adler-32");

create_info_struct!(
    Adler32CheckSumInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
