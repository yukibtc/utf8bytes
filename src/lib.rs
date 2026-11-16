#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::str::Utf8Error;
use core::{fmt, str};

use bytes::Bytes;

/// A cheaply cloneable UTF-8 chunk of bytes.
#[derive(Debug, Clone)]
pub struct Utf8Bytes(Bytes);

impl PartialEq for Utf8Bytes {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Utf8Bytes {}

impl PartialOrd for Utf8Bytes {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Utf8Bytes {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Hash for Utf8Bytes {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl fmt::Display for Utf8Bytes {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Utf8Bytes {
    /// Creates from a static str.
    #[cfg(test)]
    pub const fn from_static(str: &'static str) -> Self {
        Self(Bytes::from_static(str.as_bytes()))
    }

    /// Creates instance from slice, by copying it.
    pub fn copy_from_slice(data: &str) -> Self {
        let bytes: Bytes = Bytes::copy_from_slice(data.as_bytes());
        // SAFETY: bytes was created from a UTF-8 string
        unsafe { Utf8Bytes::from_bytes_unchecked(bytes) }
    }

    /// Creates from a [`Bytes`] object checking the encoding.
    #[inline]
    pub fn from_bytes(bytes: Bytes) -> Result<Self, Utf8Error> {
        str::from_utf8(&bytes)?;
        Ok(Self(bytes))
    }

    /// Creates from a [`Bytes`] object without checking the encoding.
    ///
    /// # Safety
    ///
    /// The bytes passed in must be valid UTF-8.
    #[inline]
    pub unsafe fn from_bytes_unchecked(bytes: Bytes) -> Self {
        Self(bytes)
    }

    /// Returns as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        // SAFETY: is valid uft8
        unsafe { str::from_utf8_unchecked(&self.0) }
    }
}

impl Deref for Utf8Bytes {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

#[cfg(feature = "alloc")]
impl From<String> for Utf8Bytes {
    #[inline]
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

#[cfg(feature = "alloc")]
impl From<&String> for Utf8Bytes {
    #[inline]
    fn from(s: &String) -> Self {
        s.as_str().into()
    }
}

impl From<&str> for Utf8Bytes {
    #[inline]
    fn from(s: &str) -> Self {
        Self(Bytes::copy_from_slice(s.as_bytes()))
    }
}

impl From<Utf8Bytes> for Bytes {
    #[inline]
    fn from(bytes: Utf8Bytes) -> Self {
        bytes.0
    }
}

impl TryFrom<Bytes> for Utf8Bytes {
    type Error = Utf8Error;

    #[inline]
    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Self::from_bytes(bytes)
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for Utf8Bytes {
    type Error = Utf8Error;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Bytes::from(bytes).try_into()
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use std::collections::hash_map::DefaultHasher;

    use super::*;

    fn hash<T>(value: T) -> u64
    where
        T: Hash,
    {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn hash_consistency() {
        let bytes = Utf8Bytes::from_static("hash_consistency");

        let string: &str = bytes.as_str();

        assert_eq!(hash(string), hash(bytes));
    }
}
