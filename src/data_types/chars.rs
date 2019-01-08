//! UEFI character handling
//!
//! UEFI uses both Latin-1 and UCS-2 character encoding, this module implements
//! support for the associated character types.

use core::convert::{TryFrom, TryInto};
use core::fmt::{self, Debug, Display};

/// Trait for operating on UEFI character types generically
///
/// This trait features a fairly extensive subset of the traits implemented by
/// `char` (which can be extended towards the full set if needed), together with
/// traits for converting to and from char and the underlying integer type.
///
/// If the conversion from the underlying integer type is faillible, it should
/// use the `CharConversionError` error type below. Otherwise, it may use `!`.
pub trait Character:
    Clone
    + Copy
    + Debug
    + Default
    + Display
    + Eq
    + Into<char>
    + Into<<Self as Character>::IntRepr>
    + PartialEq
    + PartialOrd
    + Ord
    + Send
    + Sync
    + TryFrom<char, Error = CharConversionError>
    + TryFrom<<Self as Character>::IntRepr>
{
    /// Integer representation of this character type
    type IntRepr;

    /// The NUL character for this character type, used to terminate C strings
    const NUL: Self;

    /// The suggested replacement character for this type
    ///
    /// This character should be used as a placeholder when a character
    /// conversion error occurs and aborting the operation is not possible.
    ///
    /// If the Unicode replacement character \u{fffd} if supported, it should be
    /// used, otherwise any reasonable approximation will do.
    const REPLACEMENT: Self;
}

/// Error type used for faillible character conversions
pub enum CharConversionError {
    /// Input is a valid Unicode code point, but too wide for this type
    TooWide,

    /// Input is not a valid Unicode code point
    InvalidChar,
}

/// A Latin-1 character
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Char8(u8);

impl TryFrom<char> for Char8 {
    type Error = CharConversionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let code_point = value as u32;
        if code_point <= 0xff {
            Ok(Char8(code_point as u8))
        } else {
            Err(CharConversionError::TooWide)
        }
    }
}

impl Into<char> for Char8 {
    fn into(self) -> char {
        self.0 as char
    }
}

impl From<u8> for Char8 {
    fn from(value: u8) -> Self {
        Char8(value)
    }
}

impl Into<u8> for Char8 {
    fn into(self) -> u8 {
        self.0
    }
}

impl Debug for Char8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <char as Debug>::fmt(&From::from(self.0), f)
    }
}

impl Display for Char8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <char as Display>::fmt(&From::from(self.0), f)
    }
}

impl Character for Char8 {
    type IntRepr = u8;
    const NUL: Self = Char8(0);
    const REPLACEMENT: Self = Char8(b'?');
}

/// An UCS-2 code point
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Char16(u16);

impl TryFrom<char> for Char16 {
    type Error = CharConversionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let code_point = value as u32;
        if code_point <= 0xffff {
            Ok(Char16(code_point as u16))
        } else {
            Err(CharConversionError::TooWide)
        }
    }
}

impl Into<char> for Char16 {
    fn into(self) -> char {
        u32::from(self.0).try_into().unwrap()
    }
}

impl TryFrom<u16> for Char16 {
    type Error = CharConversionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        // We leverage char's TryFrom<u32> impl for Unicode validity checking
        let res: Result<char, _> = u32::from(value).try_into();
        if let Ok(ch) = res {
            ch.try_into()
        } else {
            Err(CharConversionError::InvalidChar)
        }
    }
}

impl Into<u16> for Char16 {
    fn into(self) -> u16 {
        self.0
    }
}

impl Debug for Char16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(c) = u32::from(self.0).try_into() {
            <char as Debug>::fmt(&c, f)
        } else {
            write!(f, "Char16({:?})", self.0)
        }
    }
}

impl Display for Char16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(c) = u32::from(self.0).try_into() {
            <char as Display>::fmt(&c, f)
        } else {
            write!(f, "{}", core::char::REPLACEMENT_CHARACTER)
        }
    }
}

impl Character for Char16 {
    type IntRepr = u16;
    const NUL: Self = Char16(0);
    const REPLACEMENT: Self = Char16(0xfffd); // �
}
