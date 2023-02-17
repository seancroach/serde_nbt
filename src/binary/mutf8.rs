use alloc::{borrow::Cow, str::from_utf8, string::String, vec::Vec};

#[inline]
pub(super) fn decode(bytes: &[u8]) -> zc_io::Result<Cow<str>> {
    from_utf8(bytes)
        .map(Cow::Borrowed)
        .or_else(|_| decode_mutf8(bytes).map(Cow::Owned))
}

#[inline(never)]
#[cold]
#[allow(clippy::unnested_or_patterns)] // this hurts readability otherwise
fn decode_mutf8(bytes: &[u8]) -> zc_io::Result<String> {
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut iter = bytes.iter();

    macro_rules! err {
        () => {{
            return Err(zc_io::error!(InvalidData, "invalid MUTF-8 data"));
        }};
    }

    macro_rules! next {
        () => {
            match iter.next() {
                Some(&byte) => byte,
                None => err!(),
            }
        };
    }

    macro_rules! next_continuation {
        () => {{
            let byte = next!();
            if is_continuation_byte(byte) {
                byte
            } else {
                err!();
            }
        }};
    }

    while let Some(&first) = iter.next() {
        if first == NULL_PAIR[0] {
            match iter.next() {
                Some(&byte) if byte == NULL_PAIR[1] => {
                    decoded.push(NULL_CODE_POINT);
                }
                _ => err!(),
            }
        } else if first <= MAX_ASCII_CODE_POINT {
            decoded.push(first);
        } else {
            let width = match utf8_char_width(first) {
                Some(v) => v,
                None => err!(),
            };
            let second = next_continuation!();
            match width {
                2 => decoded.extend_from_slice(&[first, second]),
                3 => {
                    let third = next_continuation!();
                    match (first, second) {
                        (0xE0, 0xA0..=0xBF)
                        | (0xE1..=0xEC, 0x80..=0xBF)
                        | (0xED, 0x80..=0x9F)
                        | (0xEE..=0xEF, 0x80..=0xBF) => {
                            decoded.extend_from_slice(&[first, second, third]);
                        }
                        (0xED, 0xA0..=0xAF) => {
                            let fourth = next!();
                            if fourth != 0xED {
                                err!();
                            }
                            let fifth = next_continuation!();
                            if !(0xB0..=0xBF).contains(&fifth) {
                                err!();
                            }
                            let sixth = next_continuation!();
                            decoded.extend_from_slice(&decode_surrogate_pair(
                                second, third, fifth, sixth,
                            ));
                        }
                        _ => err!(),
                    }
                }
                _ => err!(),
            }
        }
    }

    debug_assert!(from_utf8(&decoded).is_ok());
    Ok(unsafe { String::from_utf8_unchecked(decoded) })
}

#[inline]
fn decode_surrogate_pair(second: u8, third: u8, fifth: u8, sixth: u8) -> [u8; 4] {
    let surrogate1 = decode_surrogate(second, third);
    let surrogate2 = decode_surrogate(fifth, sixth);
    let code_point = 0x10000 + ((surrogate1 - 0xD800) << 10 | (surrogate2 - 0xDC00));
    decode_code_point(code_point)
}

#[inline]
#[allow(clippy::cast_lossless)]
fn decode_surrogate(second: u8, third: u8) -> u32 {
    const VAL_MASK: u8 = 0b0011_1111;
    0xD000 | ((second & VAL_MASK) as u32) << 6 | (third & VAL_MASK) as u32
}

#[inline]
fn decode_code_point(code_point: u32) -> [u8; 4] {
    const STRT_TAG: u8 = 0b1111_0000;
    [
        STRT_TAG | ((code_point & 0b1_1100_0000_0000_0000_0000) >> 18) as u8,
        CONT_TAG | ((code_point & 0b0_0011_1111_0000_0000_0000) >> 12) as u8,
        CONT_TAG | ((code_point & 0b0_0000_0000_1111_1100_0000) >> 6) as u8,
        CONT_TAG | ((code_point & 0b0_0000_0000_0000_0011_1111) as u8),
    ]
}

#[must_use]
#[inline]
pub(super) fn encode(str: &str) -> Cow<[u8]> {
    if is_valid(str) {
        Cow::Borrowed(str.as_bytes())
    } else {
        Cow::Owned(encode_mutf8(str))
    }
}

#[must_use]
#[inline(never)]
#[cold]
fn encode_mutf8(str: &str) -> Vec<u8> {
    let bytes = str.as_bytes();
    let capacity = len(str);
    let mut encoded = Vec::with_capacity(capacity);
    let mut index = 0;

    while index < bytes.len() {
        let byte = bytes[index];
        if byte == NULL_CODE_POINT {
            encoded.extend_from_slice(&NULL_PAIR);
            index += 1;
        } else if byte <= MAX_ASCII_CODE_POINT {
            encoded.push(byte);
            index += 1;
        } else {
            let width = utf8_char_width(byte).unwrap();
            let slice_range = index..index + width;
            if width <= MUTF8_MAX_CHAR_WIDTH {
                encoded.extend(&bytes[slice_range]);
            } else {
                let str = &str[slice_range];
                let code_point = str.chars().next().unwrap() as u32;
                let surrogate_pair = to_surrogate_pair(code_point);
                let encoded_pair = encode_surrogate_pair(surrogate_pair);
                encoded.extend(&encoded_pair);
            }
            index += width;
        }
    }

    encoded
}

const NULL_PAIR: [u8; 2] = [0xC0, 0x80];

#[inline]
fn encode_surrogate_pair(surrogate_pair: [u16; 2]) -> [u8; 6] {
    let [b1, b2, b3] = encode_surrogate(surrogate_pair[0]);
    let [b4, b5, b6] = encode_surrogate(surrogate_pair[1]);
    [b1, b2, b3, b4, b5, b6]
}

#[inline]
fn encode_surrogate(surrogate: u16) -> [u8; 3] {
    const STRT_TAG: u8 = 0b1110_0000;
    [
        STRT_TAG | ((surrogate & 0b1111_0000_0000_0000) >> 12) as u8,
        CONT_TAG | ((surrogate & 0b0000_1111_1100_0000) >> 6) as u8,
        CONT_TAG | ((surrogate & 0b0000_0000_0011_1111) as u8),
    ]
}

#[inline]
#[allow(clippy::cast_possible_truncation)]
fn to_surrogate_pair(code_point: u32) -> [u16; 2] {
    let code_point = code_point - 0x10000;
    let first = ((code_point >> 10) as u16) | 0xD800;
    let second = ((code_point & 0x3FF) as u16) | 0xDC00;
    [first, second]
}

#[must_use]
fn len(str: &str) -> usize {
    let bytes = str.as_bytes();
    let mut len = 0;
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == NULL_CODE_POINT {
            len += 2;
            index += 1;
        } else if byte <= MAX_ASCII_CODE_POINT {
            len += 1;
            index += 1;
        } else {
            // SAFETY: Valid UTF-8 will never yield a `None` value:
            let width = unsafe { utf8_char_width(byte).unwrap_unchecked() };
            len += if width <= MUTF8_MAX_CHAR_WIDTH {
                width
            } else {
                6
            };
            index += width;
        }
    }
    len
}

#[must_use]
fn is_valid(str: &str) -> bool {
    for byte in str.bytes() {
        if is_continuation_byte(byte) {
            continue;
        }

        if byte == NULL_CODE_POINT {
            return false;
        }

        if let Some(width) = utf8_char_width(byte) {
            if width > MUTF8_MAX_CHAR_WIDTH {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

const NULL_CODE_POINT: u8 = 0x00;
const MUTF8_MAX_CHAR_WIDTH: usize = 3;

#[must_use]
#[inline]
fn is_continuation_byte(byte: u8) -> bool {
    const TAG_MASK: u8 = 0b1100_0000;
    byte & TAG_MASK == CONT_TAG
}

const CONT_TAG: u8 = 0b1000_0000;

#[must_use]
fn utf8_char_width(byte: u8) -> Option<usize> {
    match byte {
        0x00..=MAX_ASCII_CODE_POINT => Some(1),
        0xC2..=0xDF => Some(2),
        0xE0..=0xEF => Some(3),
        0xF0..=0xF4 => Some(4),
        _ => None,
    }
}

const MAX_ASCII_CODE_POINT: u8 = 0x7F;
