use core::fmt;

/// The error type for decoding a emoji256 string into `Vec<u8>` or `[u8; N]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FromEmoji256Error {
    /// An invalid char was found.
    InvalidUtf8 {
        index: usize,
    },

    // The char did not have a valid index in the table. Valid emojis listed at `table.rs`.
    InvalidEmoji256 {
        index: usize,
        c: char,
    },

    // All Emoji256 strings are a multiple of 4 bytes long.
    InvalidSrcLength,

    /// If the emoji256 string is decoded into a fixed sized container, such as an
    /// array, the container's length has to be emoji256 string's length * 4.
    InvalidDestLength,
}

#[cfg(feature = "std")]
impl std::error::Error for FromEmoji256Error {}

impl fmt::Display for FromEmoji256Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FromEmoji256Error::InvalidUtf8 { index } => {
                write!(f, "Invalid character at position {}", index)
            }
            FromEmoji256Error::InvalidEmoji256 { index, c } => {
                write!(f, "Invalid character {:?} at position {}", c, index)
            }
            FromEmoji256Error::InvalidSrcLength => {
                write!(f, "Source buffer length is not multiple of 4")
            }
            FromEmoji256Error::InvalidDestLength => {
                write!(f, "Destination buffer length is not multiple of 4.")
            }
        }
    }
}

#[cfg(test)]
// this feature flag is here to suppress unused
// warnings of `super::*` and `pretty_assertions::assert_eq`
#[cfg(feature = "alloc")]
mod tests {
    use super::*;
    #[cfg(feature = "alloc")]
    use alloc::string::ToString;
    use pretty_assertions::assert_eq;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_display() {
        assert_eq!(
            FromEmoji256Error::InvalidUtf8 { index: 0 }.to_string(),
            "Invalid character at position 0"
        );

        assert_eq!(
            FromEmoji256Error::InvalidEmoji256 { index: 5, c: '\n' }.to_string(),
            "Invalid character '\\n' at position 5"
        );

        assert_eq!(
            FromEmoji256Error::InvalidSrcLength.to_string(),
            "Source buffer length is not multiple of 4"
        );

        assert_eq!(
            FromEmoji256Error::InvalidDestLength.to_string(),
            "Destination buffer length is not multiple of 4."
        );
    }
}
