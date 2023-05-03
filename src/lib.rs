//! Encoding and decoding emoji256 strings.
//!
//! For most cases, you can simply use the [`decode`] and [`encode`] functions.
//! If you need a bit more control, use the traits [`ToEmoji256`] and [`FromEmoji256`] instead.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "alloc")]
//! let hello_world = emoji256::encode("Hello world!");
//!
//! println!("{}", hello_world); // Prints "👊💛💣💣💦🍻💸💦💩💣💚🍼"
//! # assert_eq!(hello_world, "👊💛💣💣💦🍻💸💦💩💣💚🍼");
//! ```

#![doc(html_root_url = "https://docs.rs/emoji256/0.1.0")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::unreadable_literal)]
#![feature(slice_as_chunks)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

use core::{cmp::Ordering, iter, str};

mod error;
pub use crate::error::FromEmoji256Error;

mod table;
use crate::table::EMOJI256;

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;
#[cfg(feature = "serde")]
pub use crate::serde::deserialize;
#[cfg(all(feature = "alloc", feature = "serde"))]
pub use crate::serde::serialize;

/// Encoding values as emoji256 string.
///
/// This trait is implemented for all `T` which implement `AsRef<[u8]>`. This
/// includes `String`, `str`, `Vec<u8>` and `[u8]`.
///
/// # Example
///
/// ```
/// use emoji256::ToEmoji256;
///
/// println!("{}", "Hello world!".encode_emoji256::<String>());  // prints "👊💛💣💣💦🍻💸💦💩💣💚🍼"
/// # assert_eq!("Hello world!".encode_emoji256::<String>(), "👊💛💣💣💦🍻💸💦💩💣💚🍼");
/// ```
///
/// *Note*: instead of using this trait, you might want to use [`encode()`].
pub trait ToEmoji256 {
    fn encode_emoji256<T: iter::FromIterator<char>>(&self) -> T;
}

impl<T: AsRef<[u8]>> ToEmoji256 for T {
    fn encode_emoji256<U: iter::FromIterator<char>>(&self) -> U {
        self.as_ref()
            .iter()
            .map(|byte| EMOJI256[*byte as usize])
            .collect()
    }
}

/// Encodes `data` as emoji256 string.
///
/// # Example
///
/// ```
/// # assert_eq!(emoji256::encode("Hello world!"), "👊💛💣💣💦🍻💸💦💩💣💚🍼");
/// emoji256::encode("Hello world!");  // => "👊💛💣💣💦🍻💸💦💩💣💚🍼"
/// # assert_eq!(emoji256::encode(vec![1, 2, 3, 15, 16]), "🌈🌊🌙🌼🌿");
/// emoji256::encode(vec![1, 2, 3, 15, 16]);  // => "🌈🌊🌙🌼🌿"
/// ```
#[must_use]
#[cfg(feature = "alloc")]
pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
    data.encode_emoji256()
}

/// Encodes some bytes into a mutable slice of bytes.
///
/// The output buffer has to be able to hold at least `input.len() * 4` bytes,
/// otherwise this function will return an error.
///
/// # Example
///
/// ```
/// # use emoji256::FromEmoji256Error;
/// # fn main() -> Result<(), FromEmoji256Error> {
/// let mut bytes = [0u8; 4 * 4];
/// emoji256::encode_to_slice(b"kiwi", &mut bytes)?;
/// assert_eq!(&bytes, "💢💟💸💟".as_bytes());
/// # Ok(())
/// # }
/// ```
pub fn encode_to_slice<T: AsRef<[u8]>>(
    input: T,
    output: &mut [u8],
) -> Result<(), FromEmoji256Error> {
    if input.as_ref().len() * 4 != output.len() {
        return Err(FromEmoji256Error::InvalidDestLength);
    }

    let skip_by_4 = (0..output.len()).step_by(4);
    for (byte, i) in input.as_ref().iter().zip(skip_by_4) {
        let char = EMOJI256[*byte as usize];
        char.encode_utf8(&mut output[i..i + 4]);
    }

    Ok(())
}

/// Types that can be decoded from a emoji256 string.
///
/// This trait is implemented for `Vec<u8>` and small `u8`-arrays.
///
/// # Example
///
/// ```
/// use core::str;
/// use emoji256::FromEmoji256;
///
/// let buffer = <[u8; 12]>::from_emoji256("👊💛💣💣💦🍻💸💦💩💣💚🍼")?;
/// let string = str::from_utf8(&buffer).unwrap();
///
/// println!("{}", string); // prints "Hello world!"
/// # assert_eq!("Hello world!", string);
/// # Ok::<(), emoji256::FromEmoji256Error>(())
/// ```
pub trait FromEmoji256: Sized {
    type Error;

    /// Creates an instance of type `Self` from the given emoji256 string, or fails
    /// with a custom error type.
    fn from_emoji256<T: AsRef<[u8]>>(emoji256: T) -> Result<Self, Self::Error>;
}

fn emo2byte(c: char, table: &[char; 256]) -> Option<u8> {
    binary_search(c, table, 0, table.len())
}
fn binary_search(c: char, table: &[char; 256], low: usize, high: usize) -> Option<u8> {
    if low >= high {
        return None;
    }

    let mid = low + (high - low) / 2;
    match c.cmp(&table[mid]) {
        Ordering::Less => binary_search(c, table, low, mid),
        Ordering::Equal => Some(mid as u8),
        Ordering::Greater => binary_search(c, table, mid + 1, high),
    }
}

#[cfg(feature = "alloc")]
impl FromEmoji256 for Vec<u8> {
    type Error = FromEmoji256Error;

    fn from_emoji256<T: AsRef<[u8]>>(data: T) -> Result<Self, Self::Error> {
        let data = data.as_ref();
        let mut bytes = vec![0; data.len() / 4];
        decode_to_slice(data, bytes.as_mut_slice())?;
        Ok(bytes)
    }
}

// Helper macro to implement the trait for a few fixed sized arrays. Once Rust
// has type level integers, this should be removed.
macro_rules! from_emoji256_array_impl {
    ($($len:expr)+) => {$(
        impl FromEmoji256 for [u8; $len] {
            type Error = FromEmoji256Error;

            fn from_emoji256<T: AsRef<[u8]>>(emoji256: T) -> Result<Self, Self::Error> {
                let mut out = [0_u8; $len];
                decode_to_slice(emoji256, &mut out as &mut [u8])?;
                Ok(out)
            }
        }
    )+}
}

from_emoji256_array_impl! {
    1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
    17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
    33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48
    49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64
    65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80
    81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96
    97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112
    113 114 115 116 117 118 119 120 121 122 123 124 125 126 127 128
    160 192 200 224 256 384 512 768 1024 2048 4096 8192 16384 32768
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
from_emoji256_array_impl! {
    65536 131072 262144 524288 1048576 2097152 4194304 8388608
    16777216 33554432 67108864 134217728 268435456 536870912
    1073741824 2147483648
}

#[cfg(target_pointer_width = "64")]
from_emoji256_array_impl! {
    4294967296
}

/// Decodes a emoji256 string into raw bytes.
///
/// # Example
///
/// ```
/// # assert_eq!(
/// #     emoji256::decode("👊💛💣💣💦🍻💸💦💩💣💚🍼"),
/// #     Ok("Hello world!".to_owned().into_bytes())
/// # );
/// emoji256::decode("👊💛💣💣💦🍻💸💦💩💣💚🍼");  // => "Hello world!"
/// # assert_eq!(emoji256::decode("123"), Err(emoji256::FromEmoji256Error::InvalidSrcLength));
/// emoji256::decode("123");  // => Err(FromEmoji256Error::InvalidSrcLength)
/// # assert!(emoji256::decode("foo").is_err());
/// emoji256::decode("foo");  // => Err(FromEmoji256Error::InvalidEmoji256)
/// ```
#[cfg(feature = "alloc")]
pub fn decode<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>, FromEmoji256Error> {
    FromEmoji256::from_emoji256(data)
}

/// Decode a emoji256 string into a mutable bytes slice.
///
/// # Example
///
/// ```
/// let mut bytes = [0u8; 4];
/// # assert_eq!(emoji256::decode_to_slice("💢💟💸💟", &mut bytes as &mut [u8]), Ok(()));
/// emoji256::decode_to_slice("💢💟💸💟", &mut bytes as &mut [u8]);
/// assert_eq!(&bytes, b"kiwi");
/// ```
pub fn decode_to_slice<T: AsRef<[u8]>>(data: T, out: &mut [u8]) -> Result<(), FromEmoji256Error> {
    let data = data.as_ref();
    if data.len() % 4 != 0 {
        return Err(FromEmoji256Error::InvalidSrcLength);
    }
    if data.len() / 4 != out.len() {
        return Err(FromEmoji256Error::InvalidDestLength);
    }

    for (src, dest) in iter::zip(
        str::from_utf8(data)
            .map_err(|e| FromEmoji256Error::InvalidUtf8 {
                index: e.valid_up_to() + 1,
            })?
            .chars()
            .enumerate()
            .map(|(i, c)| {
                emo2byte(c, EMOJI256).ok_or(FromEmoji256Error::InvalidEmoji256 { index: i, c })
            }),
        out.iter_mut(),
    ) {
        *dest = src?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[cfg(feature = "alloc")]
    use alloc::string::ToString;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_encode_to_slice() {
        let mut output_1 = [0; 4 * 4];
        encode_to_slice(b"kiwi", &mut output_1).unwrap();
        assert_eq!(&output_1, "💢💟💸💟".as_bytes());

        let mut output_2 = [0; 5 * 4];
        encode_to_slice(b"kiwis", &mut output_2).unwrap();
        assert_eq!(&output_2, "💢💟💸💟💪".as_bytes());

        let mut output_3 = [0; 100];

        assert_eq!(
            encode_to_slice(b"kiwis", &mut output_3),
            Err(FromEmoji256Error::InvalidDestLength)
        );
    }

    #[test]
    fn test_decode_to_slice() {
        let mut output_1 = [0; 4];
        decode_to_slice("💢💟💸💟", &mut output_1).unwrap();
        assert_eq!(&output_1, b"kiwi");

        let mut output_2 = [0; 5];
        decode_to_slice("💢💟💸💟💪", &mut output_2).unwrap();
        assert_eq!(&output_2, b"kiwis");

        let mut output_3 = [0; 4];

        assert_eq!(
            decode_to_slice(b"6", &mut output_3),
            Err(FromEmoji256Error::InvalidSrcLength)
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_encode() {
        assert_eq!(encode("foobar"), "💜💦💦💘💗💩");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_decode() {
        assert_eq!(
            decode("💜💦💦💘💗💩"),
            Ok(String::from("foobar").into_bytes())
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    pub fn test_from_emoji256_okay_str() {
        assert_eq!(Vec::from_emoji256("💜💦💦💘💗💩").unwrap(), b"foobar");
    }

    #[test]
    #[cfg(feature = "alloc")]
    pub fn test_from_emoji256_okay_bytes() {
        assert_eq!(Vec::from_emoji256("💜💦💦💘💗💩").unwrap(), b"foobar");
    }

    #[test]
    #[cfg(feature = "alloc")]
    pub fn test_invalid_length() {
        assert_eq!(
            Vec::from_emoji256("1").unwrap_err(),
            FromEmoji256Error::InvalidSrcLength
        );
        assert_eq!(
            Vec::from_emoji256("💜💦💦💘💗💩1").unwrap_err(),
            FromEmoji256Error::InvalidSrcLength
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    pub fn test_invalid_char() {
        assert_eq!(
            Vec::from_emoji256("💜💦66ag").unwrap_err(),
            FromEmoji256Error::InvalidEmoji256 { c: '6', index: 2 }
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    pub fn test_empty() {
        assert_eq!(Vec::from_emoji256("").unwrap(), b"");
    }

    #[test]
    #[cfg(feature = "alloc")]
    pub fn test_from_emoji256_whitespace() {
        assert_eq!(
            Vec::from_emoji256("💜💦    💦💘💗💩").unwrap_err(),
            FromEmoji256Error::InvalidEmoji256 { c: ' ', index: 2 }
        );
    }

    #[test]
    pub fn test_from_emoji256_array() {
        assert_eq!(
            <[u8; 6] as FromEmoji256>::from_emoji256("💜💦💦💘💗💩"),
            Ok([0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72])
        );

        assert_eq!(
            <[u8; 5] as FromEmoji256>::from_emoji256("💜💦💦💘💗💩"),
            Err(FromEmoji256Error::InvalidDestLength)
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_to_emoji256() {
        assert_eq!(
            [0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72].encode_emoji256::<String>(),
            "💜💦💦💘💗💩".to_string(),
        );
    }
}
