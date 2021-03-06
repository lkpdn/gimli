//! Types for compile-time and run-time endianity.

use byteorder;
use byteorder::ByteOrder;
use std::fmt::Debug;

/// A trait describing the endianity of some buffer.
pub trait Endianity: Debug + Default + Clone + Copy + PartialEq + Eq {
    /// Return true for big endian byte order.
    fn is_big_endian(self) -> bool;

    /// Return true for little endian byte order.
    #[inline]
    fn is_little_endian(self) -> bool {
        !self.is_big_endian()
    }

    /// Reads an unsigned 16 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 2`.
    #[inline]
    fn read_u16(self, buf: &[u8]) -> u16 {
        if self.is_big_endian() {
            byteorder::BigEndian::read_u16(buf)
        } else {
            byteorder::LittleEndian::read_u16(buf)
        }
    }

    /// Reads an unsigned 32 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    #[inline]
    fn read_u32(self, buf: &[u8]) -> u32 {
        if self.is_big_endian() {
            byteorder::BigEndian::read_u32(buf)
        } else {
            byteorder::LittleEndian::read_u32(buf)
        }
    }

    /// Reads an unsigned 64 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    #[inline]
    fn read_u64(self, buf: &[u8]) -> u64 {
        if self.is_big_endian() {
            byteorder::BigEndian::read_u64(buf)
        } else {
            byteorder::LittleEndian::read_u64(buf)
        }
    }

    /// Reads a signed 16 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 2`.
    #[inline]
    fn read_i16(self, buf: &[u8]) -> i16 {
        self.read_u16(buf) as i16
    }

    /// Reads a signed 32 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    #[inline]
    fn read_i32(self, buf: &[u8]) -> i32 {
        self.read_u32(buf) as i32
    }

    /// Reads a signed 64 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    #[inline]
    fn read_i64(self, buf: &[u8]) -> i64 {
        self.read_u64(buf) as i64
    }

    /// Writes an unsigned 64 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    #[inline]
    fn write_u64(self, buf: &mut [u8], n: u64) {
        if self.is_big_endian() {
            byteorder::BigEndian::write_u64(buf, n)
        } else {
            byteorder::LittleEndian::write_u64(buf, n)
        }
    }
}

/// Byte order that is selectable at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RunTimeEndian {
    /// Little endian byte order.
    Little,
    /// Big endian byte order.
    Big,
}

impl Default for RunTimeEndian {
    #[cfg(target_endian = "little")]
    #[inline]
    fn default() -> RunTimeEndian {
        RunTimeEndian::Little
    }

    #[cfg(target_endian = "big")]
    #[inline]
    fn default() -> RunTimeEndian {
        RunTimeEndian::Big
    }
}

impl Endianity for RunTimeEndian {
    #[inline]
    fn is_big_endian(self) -> bool {
        self != RunTimeEndian::Little
    }
}

/// Little endian byte order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LittleEndian;

impl Default for LittleEndian {
    #[inline]
    fn default() -> LittleEndian {
        LittleEndian
    }
}

impl Endianity for LittleEndian {
    #[inline]
    fn is_big_endian(self) -> bool {
        false
    }
}

/// Big endian byte order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BigEndian;

impl Default for BigEndian {
    #[inline]
    fn default() -> BigEndian {
        BigEndian
    }
}

impl Endianity for BigEndian {
    #[inline]
    fn is_big_endian(self) -> bool {
        true
    }
}

/// The native endianity for the target platform.
#[cfg(target_endian = "little")]
pub type NativeEndian = LittleEndian;

#[cfg(target_endian = "little")]
#[allow(non_upper_case_globals)]
#[doc(hidden)]
pub const NativeEndian: LittleEndian = LittleEndian;

/// The native endianity for the target platform.
#[cfg(target_endian = "big")]
pub type NativeEndian = BigEndian;

#[cfg(target_endian = "big")]
#[allow(non_upper_case_globals)]
#[doc(hidden)]
pub const NativeEndian: BigEndian = BigEndian;
