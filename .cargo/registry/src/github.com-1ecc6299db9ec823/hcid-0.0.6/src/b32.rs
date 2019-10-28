//! Utilities for encoding / decoding basic base32

/// 5 bit mask
const MASK: usize = 31;

/// holochain base32 alphabet
static ALPHABET: &'static [u8] = b"ABCDEFGHIJKMNOPQRSTUVWXYZ3456789";

/// reverse lookup table for alphabet positioning (ascii - 51)
static REV_LOOKUP: &'static [u8] = &[
    25, 26, 27, 28, 29, 30, 31,         // 0, 1, 2, 3, 4, 5, 6,
    255, 255, 255, 255, 255, 255, 255,  // 7, 8, 9, 10, 11, 12, 13,
    0, 1, 2, 3, 4, 5, 6,                // 14, 15, 16, 17, 18, 19, 20,
    7, 8, 9, 10,                        // 21, 22, 23, 24,
    255,                                // 25,
    11, 12, 13, 14, 15, 16, 17,         // 26, 27, 28, 29, 30, 31, 32,
    18, 19, 20, 21, 22, 23, 24,         // 33, 34, 35, 36, 37, 38, 39,
];

/// encode a byte buffer into basic holochain base32
pub fn encode (data: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];

    let mut bits: usize = 0;
    let mut tmp: usize = 0;

    for c in data {
        tmp = (tmp << 8) | (0xff & c) as usize;
        bits += 8;

        while bits > 5 {
            bits -= 5;
            out.push(ALPHABET[MASK & (tmp >> bits)])
        }
    }

    if bits > 0 {
        out.push(ALPHABET[MASK & (tmp << (5 - bits))])
    }

    out
}

/// decode an already sanitized holochain base32 string into a byte buffer
pub fn decode (data: &[u8]) -> crate::HcidResult<Vec<u8>> {
    let mut out: Vec<u8> = vec![];

    let mut bits: usize = 0;
    let mut tmp: usize = 0;

    for c in data {
        if c < &51 || *c as usize >= REV_LOOKUP.len() + 51 {
            return Err("bad input".into());
        }

        let v: usize = REV_LOOKUP[(c - 51) as usize] as usize;

        if v == 255 {
            return Err("bad input".into());
        }

        tmp = (tmp << 5) | v;
        bits += 5;

        if bits >= 8 {
            bits -= 8;
            out.push((0xff & (tmp >> bits)) as u8)
        }
    }

    if bits >= 5 || (0xff & (tmp << (8 - bits))) != 0 {
        return Err("unexpected eof".into());
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn both_ways(e: &[u8], d: &[u8]) {
        assert_eq!(d[..], decode(e).unwrap()[..]);
        assert_eq!(e[..], encode(d)[..]);
    }

    #[test]
    fn it_should_encode_and_decode_1() {
        both_ways(
            b"AEBAGBASWMGQ9VRB",
            &[1, 2, 3, 4, 17, 170, 204, 255, 210, 1]);
    }

    #[test]
    fn it_should_encode_and_decode_2() {
        both_ways(
            b"AEBAGBASWMGQ9VR",
            &[1, 2, 3, 4, 17, 170, 204, 255, 210]);
    }

    #[test]
    fn it_should_encode_and_decode_3() {
        both_ways(
            b"AEBAGBASWMGA",
            &[1, 2, 3, 4, 17, 170, 204]);
    }

    #[test]
    fn it_should_encode_and_decode_4() {
        both_ways(
            b"ABCDEFGHIJKMNOPQRSTUVWXYZ3456789",
            &[0, 68, 50, 20, 199, 66, 84, 182, 53, 207, 132, 101, 58, 86, 215, 198, 117, 190, 119, 223]);
    }

    #[test]
    fn it_should_error_on_bad_decode() {
        assert_eq!(
            "HcidError(\"bad input\")",
            &format!("{}", decode(b"A%BAGBASWMGA").unwrap_err()));
    }
}
