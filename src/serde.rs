//! Emoji256 encoding with `serde`.
#[cfg_attr(
    all(feature = "alloc", feature = "serde"),
    doc = r##"
# Example

```
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Foo {
    #[serde(with = "emoji256")]
    bar: Vec<u8>,
}
```
"##
)]
use serde::de::{Error, Visitor};
use serde::Deserializer;
#[cfg(feature = "alloc")]
use serde::Serializer;

#[cfg(feature = "alloc")]
use alloc::string::String;

use core::fmt;
use core::marker::PhantomData;

use crate::FromEmoji256;

#[cfg(feature = "alloc")]
use crate::ToEmoji256;

/// Serializes `data` as emoji256 string using lowercase characters.
///
/// The resulting string's length is always a multiple of 4, each byte in data is always
/// encoded using one emoji256 digit which is equal to one 4-byte UTF8 codepoint. Thus,
/// the resulting string contains exactly four times as many bytes as the input data.
#[cfg(feature = "alloc")]
pub fn serialize<S, T>(data: T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToEmoji256,
{
    let s = data.encode_emoji256::<String>();
    serializer.serialize_str(&s)
}

/// Deserializes a emoji256 string into raw bytes.
///
/// The input string's length must be a multiple of 4, and the resulting byte vector's
/// length will always be exactly one fourth of the input string's length.
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromEmoji256,
    <T as FromEmoji256>::Error: fmt::Display,
{
    struct Emoji256StrVisitor<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for Emoji256StrVisitor<T>
    where
        T: FromEmoji256,
        <T as FromEmoji256>::Error: fmt::Display,
    {
        type Value = T;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "an emoji256 encoded string")
        }

        fn visit_str<E>(self, data: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            FromEmoji256::from_emoji256(data).map_err(Error::custom)
        }

        fn visit_borrowed_str<E>(self, data: &'de str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            FromEmoji256::from_emoji256(data).map_err(Error::custom)
        }
    }

    deserializer.deserialize_str(Emoji256StrVisitor(PhantomData))
}
