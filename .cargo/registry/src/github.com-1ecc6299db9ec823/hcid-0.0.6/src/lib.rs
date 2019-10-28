//! Holochain HCID base32 encoding utility.
//!
//! # Example
//!
//! ```
//! extern crate hcid;
//!
//! fn main() {
//!     let enc = hcid::HcidEncoding::with_kind("hcs0").unwrap();
//!     let key = enc.encode(&[0; 32]).unwrap();
//!     assert_eq!("HcSciaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", key);
//!     let buffer = enc.decode(&key).unwrap();
//!     assert_eq!([0; 32].to_vec(), buffer);
//! }
//! ```

extern crate reed_solomon;

mod error;
mod b32;
pub use error::{HcidError, HcidResult};

mod util;
use util::{b32_correct, cap_decode, cap_encode_bin, char_upper};

static HC_CODE_MAP: &'static [[u8; 2]] = &[
    [ 0xb2, 0xff ], // 51: hc30, reserved
    [ 0xb4, 0xff ], // 52: hc40, reserved
    [ 0xb6, 0xff ], // 53: hc50, reserved
    [ 0xb8, 0xff ], // 54: hc60, reserved
    [ 0xba, 0xff ], // 55: hc70, reserved
    [ 0xbc, 0xff ], // 56: hc80, reserved
    [ 0xbe, 0xff ], // 57: hc90, reserved

    // 58-61: reserved
    [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ],
    // 62-65: reserved
    [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ],
    // 66-69: reserved
    [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ],
    // 70-73: reserved
    [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ],
    // 74-77: reserved
    [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ],
    // 78-81: reserved
    [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ],
    // 82-85: reserved
    [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ],
    // 86-89: reserved
    [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ],
    // 90-93: reserved
    [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ],
    // 94-96: reserved
    [ 0xff, 0xff ], [ 0xff, 0xff ], [ 0xff, 0xff ],

    [ 0x80, 0xff ], // 97: hca0, reserved
    [ 0x82, 0xff ], // 98: hcb0, reserved
    [ 0x84, 0xff ], // 99: hcc0, reserved
    [ 0x86, 0xff ], // 100: hcd0, reserved
    [ 0x88, 0xff ], // 101: hce0, reserved
    [ 0x8a, 0xff ], // 102: hcf0, reserved
    [ 0x8c, 0xff ], // 103: hcg0, reserved
    [ 0x8e, 0xff ], // 104: hch0, reserved
    [ 0x90, 0xff ], // 105: hci0, reserved
    [ 0x92, 0xff ], // 106: hcj0, reserved
    [ 0x94, 0xff ], // 107: hck0, reserved

    [ 0xff, 0xff ], // 108: reserved, reserved

    [ 0x96, 0xff ], // 109: hcm0, reserved
    [ 0x98, 0xff ], // 110: hcn0, reserved
    [ 0x9a, 0xff ], // 111: hco0, reserved
    [ 0x9c, 0xff ], // 112: hcp0, reserved
    [ 0x9e, 0xff ], // 113: hcq0, reserved
    [ 0xa0, 0xff ], // 114: hcr0, reserved
    [ 0xa2, 0xff ], // 115: hcs0, reserved
    [ 0xa4, 0xff ], // 116: hct0, reserved
    [ 0xa6, 0xff ], // 117: hcu0, reserved
    [ 0xa8, 0xff ], // 118: hcv0, reserved
    [ 0xaa, 0xff ], // 119: hcw0, reserved
    [ 0xad, 0xff ], // 120: hcx0, reserved
    [ 0xae, 0xff ], // 121: hcy0, reserved
    [ 0xb0, 0xff ], // 122: hcz0, reserved
];

/* XXX
 *
 * HcK v0 hex:     0x389424
 * HcK v1 hex:     0x389524
 * HcA v0 hex:     0x388024
 * HcA v1 hex:     0x388124
 * HcS v0 hex:     0x38a224
 * HcS v1 hex:     0x38a324
 *
 * XXX
 */

/// represents an encoding configuration for hcid rendering and parsing
pub struct HcidEncodingConfig {
    /// byte count of actuall key data that will be encoded
    pub key_byte_count: usize,
    /// parity bytes that will be encoded directly into the base32 string (appended after key)
    pub base_parity_byte_count: usize,
    /// parity bytes that will be encoded in the alpha capitalization (appended after base parity)
    pub cap_parity_byte_count: usize,
    /// bytes to prefix before rendering to base32
    pub prefix: Vec<u8>,
    /// binary indication of the capitalization for prefix characters
    pub prefix_cap: Vec<u8>,
    /// how many characters are in a capitalization parity segment
    pub cap_segment_char_count: usize,
    /// how many characters long the fully rendered base32 string should be
    pub encoded_char_count: usize,
}

impl HcidEncodingConfig {
    /// create a new config given a kind token string
    ///
    /// # Example
    ///
    /// ```
    /// extern crate hcid;
    /// let hca0 = hcid::HcidEncodingConfig::new("hca0").unwrap();
    /// let hck0 = hcid::HcidEncodingConfig::new("hck0").unwrap();
    /// let hcs0 = hcid::HcidEncodingConfig::new("hcs0").unwrap();
    /// ```
    pub fn new(kind: &str) -> HcidResult<Self> {
        let kind_b = kind.as_bytes();
        if kind_b.len() != 4 || kind_b[0] != 104 || kind_b[1] != 99 ||
                (kind_b[3] != 48 && kind_b[3] != 49) ||
                kind_b[2] < 51 || kind_b[2] > 122 {
            return Err(format!("invalid kind: `{}`", kind).into());
        }

        let version = if kind_b[3] == 48 { 0 } else { 1 };
        let res = HC_CODE_MAP[(kind_b[2] - 51) as usize][version as usize];

        if res == 0xff {
            return Err(format!("invalid kind: `{}`", kind).into());
        }

        Ok(HcidEncodingConfig {
            key_byte_count: 32,
            base_parity_byte_count: 4,
            cap_parity_byte_count: 4,
            prefix: vec![0x38, res, 0x24],
            prefix_cap: b"101".to_vec(),
            cap_segment_char_count: 15,
            encoded_char_count: 63,
        })
    }
}

/// an instance that can encode / decode a particular hcid encoding configuration
pub struct HcidEncoding {
    config: HcidEncodingConfig,
    rs_enc: reed_solomon::Encoder,
    rs_dec: reed_solomon::Decoder,
}

impl HcidEncoding {
    /// create a new HcidEncoding instance from given HcidEncodingConfig
    pub fn new(config: HcidEncodingConfig) -> HcidResult<Self> {
        // set up a reed-solomon encoder with proper parity count
        let rs_enc = reed_solomon::Encoder::new(
            config.base_parity_byte_count + config.cap_parity_byte_count,
        );

        // set up a reed-solomon decoder with proper parity count
        let rs_dec = reed_solomon::Decoder::new(
            config.base_parity_byte_count + config.cap_parity_byte_count,
        );

        Ok(Self {
            config,
            rs_enc,
            rs_dec,
        })
    }

    /// create a new config given a kind token string
    ///
    /// # Example
    ///
    /// ```
    /// extern crate hcid;
    /// let hca0 = hcid::HcidEncoding::with_kind("hca0").unwrap();
    /// let hck0 = hcid::HcidEncoding::with_kind("hck0").unwrap();
    /// let hcs0 = hcid::HcidEncoding::with_kind("hcs0").unwrap();
    /// ```
    pub fn with_kind(kind: &str) -> HcidResult<Self> {
        HcidEncoding::new(HcidEncodingConfig::new(kind)?)
    }

    /// encode a string to base32 with this instance's configuration
    pub fn encode(&self, data: &[u8]) -> HcidResult<String> {
        if data.len() != self.config.key_byte_count {
            return Err(HcidError(String::from(format!(
                "BadDataLen:{},Expected:{}",
                data.len(),
                self.config.key_byte_count
            ))));
        }

        // generate reed-solomon parity bytes
        let full_parity = self.rs_enc.encode(data);

        // extract the bytes that will be encoded as capitalization
        let cap_bytes = &full_parity[full_parity.len() - self.config.cap_parity_byte_count..];

        // base is the bytes that will be base32 encoded
        let mut base = self.config.prefix.clone();
        base.extend_from_slice(
            &full_parity[0..full_parity.len() - self.config.cap_parity_byte_count],
        );

        // do the base32 encoding
        let mut base32 = b32::encode(&base);

        if base32.len() != self.config.encoded_char_count {
            return Err(HcidError(String::from(format!(
                "InternalGeneratedBadLen:{},Expected:{}",
                base32.len(),
                self.config.encoded_char_count
            ))));
        }

        // capitalize the prefix with a fixed scheme
        cap_encode_bin(
            &mut base32[0..self.config.prefix_cap.len()],
            &self.config.prefix_cap,
            3,
        )?;

        // iterate over segments, applying parity capitalization
        for i in 0..cap_bytes.len() {
            let seg_start = self.config.prefix_cap.len() + (i * self.config.cap_segment_char_count);
            let seg = &mut base32[seg_start..seg_start + self.config.cap_segment_char_count];
            let bin = format!("{:08b}", cap_bytes[i]).into_bytes();
            cap_encode_bin(seg, &bin, 8)?;
        }
        
        // we only use ascii characters
        // use unchecked for performance / so we don't allocate again
        unsafe {
            // return the result as a String for ease of use
            Ok(String::from_utf8_unchecked(base32))
        }
    }

    /// decode the data from a base32 string with this instance's configuration.  Reed-Solomon can
    /// correct up to 1/2 its parity size worth of erasures (if no other errors are present).
    pub fn decode(&self, data: &str) -> HcidResult<Vec<u8>> {
        // get our parsed data with erasures
        let (data, erasures) = self.pre_decode(data)?;

        if erasures.len() > ( self.config.base_parity_byte_count + self.config.cap_parity_byte_count ) / 2 {
            // our reed-solomon library makes bad corrections once erasure count exceeds 1/2 the
            // parity count (it takes 2 parity symbols to find/correct one error, 1 parity symbol to
            // correct a known erasure)
            return Err(HcidError(String::from("TooManyErrors")));
        }

        // optimise for the case where there are no transcription errors
        // this makes correcting more expensive if there *are*,
        // but on average makes the system more efficient
        if self.pre_is_corrupt(&data, &erasures)? {
            // apply reed-solomon correction
            // will "throw" on too many errors
            Ok(
                self.rs_dec.correct(&data, Some(&erasures[..]))?[0..self.config.key_byte_count]
                    .to_vec(),
            )
        } else {
            Ok(data[0..self.config.key_byte_count].to_vec())
        }
    }

    /// a lighter-weight check to determine if a base32 string is corrupt
    pub fn is_corrupt(&self, data: &str) -> HcidResult<bool> {
        // get our parsed data with erasures
        let (data, erasures) = match self.pre_decode(data) {
            Ok(v) => v,
            Err(_) => return Ok(true),
        };

        match self.pre_is_corrupt(&data, &erasures) {
            Ok(v) => Ok(v),
            Err(_) => Ok(true),
        }
    }

    /// internal helper for is_corrupt checking
    fn pre_is_corrupt(&self, data: &[u8], erasures: &[u8]) -> HcidResult<bool> {
        // if we have any erasures, we can exit early
        if erasures.len() > 0 {
            return Ok(true);
        }

        // slightly more efficient reed-solomon corruption check
        Ok(self.rs_dec.is_corrupted(&data))
    }

    /// internal helper for preparing decoding
    fn pre_decode(&self, data: &str) -> HcidResult<(Vec<u8>, Vec<u8>)> {
        if data.len() != self.config.encoded_char_count {
            return Err(HcidError(String::from(format!(
                "BadIdLen:{},Expected:{}",
                data.len(),
                self.config.encoded_char_count
            ))));
        }

        let key_base_byte_size = self.config.key_byte_count + self.config.base_parity_byte_count;
        // All char_erasures are indexed from the 0th char of the full codeword w/ prefix, but
        // byte_erasures are indexed from the 0th byte of the key+parity (ie. without the prefix).
        // Any byte of key, or base/cap parity could be erased.
        let mut byte_erasures = vec![b'0'; key_base_byte_size + self.config.cap_parity_byte_count];
        let mut char_erasures = vec![b'0'; data.len()];

        // correct any transliteration errors into our base32 alphabet
        // marking any unrecognizable characters as char-level erasures
        let mut data = b32_correct(data.as_bytes(), &mut char_erasures);

        // Pull out the parity data that was encoded as capitalization.  If its erasure,
        // determine the 
        let mut cap_bytes: Vec<u8> = Vec::new();
        let mut all_zro = true;
        let mut all_one = true;
        for i in 0..self.config.cap_parity_byte_count {
            // For cap. parity, indexing starts after pre-defined Base-32 prefix
            let char_idx = self.config.prefix_cap.len() + (i * self.config.cap_segment_char_count);
            match cap_decode(
                char_idx,
                &data[char_idx..char_idx + self.config.cap_segment_char_count],
                &char_erasures
            )? {
                None => {
                    byte_erasures[key_base_byte_size + i] = b'1';
                    cap_bytes.push(0)
                }
                Some(parity) => {
                    if all_zro && parity != 0x00_u8 {
                        all_zro = false
                    }
                    if all_one && parity != 0xFF_u8 {
                        all_one = false
                    }
                    cap_bytes.push(parity)
                }
            }
        }

        // If either all caps or all lower case (or erasure), assume the casing was lost (eg. QR
        // code, or dns segment); mark all cap-derived parity as erasures.  This allows validation
        // of codeword if all remaining parity is intact and key is correct; since no parity
        // capacity remains, no correction will be attempted.  There is only a low probability that
        // any remaining errors will be detected, in this case.  However, we're no *worse* off than
        // if we had no R-S parity at all.
        if all_zro || all_one {
            for i in 0..self.config.cap_parity_byte_count {
                byte_erasures[key_base_byte_size + i] = b'1';
            }
        }

        // we have the cap data, uppercase everything
        for c in data.iter_mut() {
            char_upper(c);
        }

        // do the base32 decode
        let mut data = b32::decode(&data)?;

        if &data[0..self.config.prefix.len()] != self.config.prefix.as_slice() {
            return Err(HcidError(String::from("PrefixMismatch")));
        }

        // remove the prefix bytes
        data.drain(0..self.config.prefix.len());

        // append our cap parity bytes
        data.append(&mut cap_bytes);

        // Sort through the char-level erasures (5 bits), and associate them with byte-level data (8
        // bits) -- in the (now prefix-free) data buffer, so that we mark the proper erasures for
        // reed-solomon correction.  Some of these chars span multiple bytes... we need to mark both.
        for i in self.config.prefix_cap.len()..char_erasures.len() {
            let c = char_erasures[i];
            if c == b'1' {
                // 1st and last bit of 5-bit segment may index different bytes
                byte_erasures[( i * 5 + 0 ) / 8 - self.config.prefix.len()] = b'1';
                byte_erasures[( i * 5 + 4 ) / 8 - self.config.prefix.len()] = b'1';
            }
        }

        // translate erasures into the form expected by our reed-solomon lib
        let mut erasures: Vec<u8> = Vec::new();
        for i in 0..byte_erasures.len() {
            if byte_erasures[i] == b'1' {
                data[i] = 0;
                erasures.push(i as u8);
            }
        }

        Ok((data, erasures))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_HEX_1: &'static str =
        "0c71db50d35d760b0ea2002ff20147c7c3a8e8030d35ef28ed1adaec9e329aba";
    static TEST_ID_1: &'static str =
        "HcKciDds5OiogymxbnHKEabQ8iavqs8dwdVaGdJW76Vp4gx47tQDfGW4OWc9w5i";

    #[test]
    fn it_encodes_1() {
        let enc = HcidEncoding::with_kind("hck0").unwrap();

        let input = hex::decode(TEST_HEX_1.as_bytes()).unwrap();
        let id = enc.encode(&input).unwrap();
        assert_eq!(TEST_ID_1, id);
    }

    #[test]
    fn it_decodes_1() {
        let enc = HcidEncoding::with_kind("hck0").unwrap();

        let data = hex::encode(&enc.decode(TEST_ID_1).unwrap());
        assert_eq!(TEST_HEX_1, data);
    }
}
