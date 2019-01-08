use super::chars::{Char16, Char8, Character};
use core::result::Result;
use core::slice;

/// Generalization of `std::ffi::CStr` to UEFI use cases
///
/// This type is heavily inspired by `std::ffi::CStr`, but extended to support
/// UEFI peculiarities such as coexistence of multiple text encoding and UCS-2.
///
/// You should refer to the documentation of `std::ffi::CStr` for more details
/// on the overall semantics. This module will only summarize them, and explain
/// where we diverge from them.
pub struct CStr<Char: Character>([Char]);

/// Errors which can occur during checked [uN] -> CStrN conversions
pub enum FromIntsWithNulError {
    /// An invalid character was encountered before the end of the slice
    InvalidChar(usize),

    /// A null character was encountered before the end of the slice
    InteriorNul(usize),

    /// The slice was not null-terminated
    NotNulTerminated,
}

impl<Char: Character> CStr<Char> {
    /// Wraps a raw UEFI string with a safe C string wrapper
    pub unsafe fn from_ptr<'ptr>(ptr: *const Char) -> &'ptr Self {
        let mut len = 0;
        while *ptr.add(len) != Char::NUL {
            len += 1
        }
        let ptr = ptr as *const Char::IntRepr;
        Self::from_ints_with_nul_unchecked(slice::from_raw_parts(ptr, len + 1))
    }

    /// Creates a C string wrapper from a nul-terminated slice of integers
    ///
    /// Unlike traditional `CStr::from_bytes_with_nul`, this function also
    /// checks character validity, as needed when handling UCS-2 data.
    pub fn from_ints_with_nul(codes: &[Char::IntRepr]) -> Result<&Self, FromIntsWithNulError> {
        for (pos, &code) in codes.iter().enumerate() {
            match Char::try_from(code) {
                // FIXME: Workaround for lack of associated consts in patterns
                Ok(c) if c == Char::NUL => {
                    if pos != codes.len() - 1 {
                        return Err(FromIntsWithNulError::InteriorNul(pos));
                    } else {
                        return Ok(unsafe { Self::from_ints_with_nul_unchecked(codes) });
                    }
                }
                Err(_) => {
                    return Err(FromIntsWithNulError::InvalidChar(pos));
                }
                _ => {}
            }
        }
        Err(FromIntsWithNulError::NotNulTerminated)
    }

    /// Unsafely creates a C string wrapper from a u16 slice.
    pub unsafe fn from_ints_with_nul_unchecked(codes: &[Char::IntRepr]) -> &Self {
        &*(codes as *const [Char::IntRepr] as *const Self)
    }

    /// Returns the inner pointer to this C string
    pub fn as_ptr(&self) -> *const Char {
        self.0.as_ptr()
    }

    /// Converts this C string to a slice of integers
    pub fn to_ints_slice(&self) -> &[Char::IntRepr] {
        let chars = self.to_ints_slice_with_nul();
        &chars[..chars.len() - 1]
    }

    /// Converts this C string to an int slice containing the trailing 0 char
    pub fn to_ints_slice_with_nul(&self) -> &[Char::IntRepr] {
        unsafe { &*(&self.0 as *const [Char] as *const [Char::IntRepr]) }
    }
}

/// A Latin-1 null-terminated string
pub type CStr8 = CStr<Char8>;

/// An UCS-2 null-terminated string
pub type CStr16 = CStr<Char16>;
