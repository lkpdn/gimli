//! Working with byte slices that have an associated endianity.

use endianity::Endianity;
use std::mem;
use std::ops::{Deref, Index, Range, RangeFrom, RangeTo};
use std::str;
use string::String;
use borrow::Cow;
use parser::{Error, Result};
use reader::Reader;

/// A `&[u8]` slice with endianity metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndianSlice<'input, Endian>
where
    Endian: Endianity,
{
    slice: &'input [u8],
    endian: Endian,
}

impl<'input, Endian> EndianSlice<'input, Endian>
where
    Endian: Endianity,
{
    /// Construct a new `EndianSlice` with the given slice and endianity.
    #[inline]
    pub fn new(slice: &'input [u8], endian: Endian) -> EndianSlice<'input, Endian> {
        EndianSlice { slice, endian }
    }

    /// Return a reference to the raw slice.
    #[inline]
    #[doc(hidden)]
    #[deprecated(note = "Method renamed to EndianSlice::slice; use that instead.")]
    pub fn buf(&self) -> &'input [u8] {
        self.slice
    }

    /// Return a reference to the raw slice.
    #[inline]
    pub fn slice(&self) -> &'input [u8] {
        self.slice
    }

    /// Split the slice in two at the given index, resulting in the tuple where
    /// the first item has range [0, idx), and the second has range [idx,
    /// len). Panics if the index is out of bounds.
    #[inline]
    pub fn split_at(&self, idx: usize) -> (EndianSlice<'input, Endian>, EndianSlice<'input, Endian>) {
        (self.range_to(..idx), self.range_from(idx..))
    }

    /// Find the first occurence of a byte in the slice, and return its index.
    #[inline]
    pub fn find(&self, byte: u8) -> Option<usize> {
        self.slice.iter().position(|ch| *ch == byte)
    }

    /// Return the offset of the start of the slice relative to the start
    /// of the given slice.
    #[inline]
    pub fn offset_from(&self, base: EndianSlice<'input, Endian>) -> usize {
        let base_ptr = base.slice.as_ptr() as *const u8 as usize;
        let ptr = self.slice.as_ptr() as *const u8 as usize;
        debug_assert!(base_ptr <= ptr);
        debug_assert!(ptr + self.slice.len() <= base_ptr + base.slice.len());
        ptr - base_ptr
    }

    /// Converts the slice to a string using `str::from_utf8`.
    ///
    /// Returns an error if the slice contains invalid characters.
    #[inline]
    pub fn to_string(&self) -> Result<&'input str> {
        str::from_utf8(self.slice).map_err(|_| Error::BadUtf8)
    }

    /// Converts the slice to a string, including invalid characters,
    /// using `String::from_utf8_lossy`.
    #[inline]
    pub fn to_string_lossy(&self) -> Cow<'input, str> {
        String::from_utf8_lossy(self.slice)
    }

    #[inline]
    fn read_slice(&mut self, len: usize) -> Result<&'input [u8]> {
        if self.slice.len() < len {
            Err(Error::UnexpectedEof)
        } else {
            let val = &self.slice[..len];
            self.slice = &self.slice[len..];
            Ok(val)
        }
    }
}

/// # Range Methods
///
/// Unfortunately, `std::ops::Index` *must* return a reference, so we can't
/// implement `Index<Range<usize>>` to return a new `EndianSlice` the way we would
/// like to. Instead, we abandon fancy indexing operators and have these plain
/// old methods.
impl<'input, Endian> EndianSlice<'input, Endian>
where
    Endian: Endianity,
{
    /// Take the given `start..end` range of the underlying slice and return a
    /// new `EndianSlice`.
    ///
    /// ```
    /// use gimli::{EndianSlice, LittleEndian};
    ///
    /// let slice = &[0x01, 0x02, 0x03, 0x04];
    /// let endian_slice = EndianSlice::new(slice, LittleEndian);
    /// assert_eq!(endian_slice.range(1..3),
    ///            EndianSlice::new(&slice[1..3], LittleEndian));
    /// ```
    pub fn range(&self, idx: Range<usize>) -> EndianSlice<'input, Endian> {
        EndianSlice {
            slice: &self.slice[idx],
            endian: self.endian,
        }
    }

    /// Take the given `start..` range of the underlying slice and return a new
    /// `EndianSlice`.
    ///
    /// ```
    /// use gimli::{EndianSlice, LittleEndian};
    ///
    /// let slice = &[0x01, 0x02, 0x03, 0x04];
    /// let endian_slice = EndianSlice::new(slice, LittleEndian);
    /// assert_eq!(endian_slice.range_from(2..),
    ///            EndianSlice::new(&slice[2..], LittleEndian));
    /// ```
    pub fn range_from(&self, idx: RangeFrom<usize>) -> EndianSlice<'input, Endian> {
        EndianSlice {
            slice: &self.slice[idx],
            endian: self.endian,
        }
    }

    /// Take the given `..end` range of the underlying slice and return a new
    /// `EndianSlice`.
    ///
    /// ```
    /// use gimli::{EndianSlice, LittleEndian};
    ///
    /// let slice = &[0x01, 0x02, 0x03, 0x04];
    /// let endian_slice = EndianSlice::new(slice, LittleEndian);
    /// assert_eq!(endian_slice.range_to(..3),
    ///            EndianSlice::new(&slice[..3], LittleEndian));
    /// ```
    pub fn range_to(&self, idx: RangeTo<usize>) -> EndianSlice<'input, Endian> {
        EndianSlice {
            slice: &self.slice[idx],
            endian: self.endian,
        }
    }
}

impl<'input, Endian> Index<usize> for EndianSlice<'input, Endian>
where
    Endian: Endianity,
{
    type Output = u8;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.slice[idx]
    }
}

impl<'input, Endian> Index<RangeFrom<usize>> for EndianSlice<'input, Endian>
where
    Endian: Endianity,
{
    type Output = [u8];
    fn index(&self, idx: RangeFrom<usize>) -> &Self::Output {
        &self.slice[idx]
    }
}

impl<'input, Endian> Deref for EndianSlice<'input, Endian>
where
    Endian: Endianity,
{
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.slice
    }
}

impl<'input, Endian> Into<&'input [u8]> for EndianSlice<'input, Endian>
where
    Endian: Endianity,
{
    fn into(self) -> &'input [u8] {
        self.slice
    }
}

impl<'input, Endian> Reader for EndianSlice<'input, Endian>
where
    Endian: Endianity,
{
    type Endian = Endian;
    type Offset = usize;

    #[inline]
    fn endian(&self) -> Endian {
        self.endian
    }

    #[inline]
    fn len(&self) -> usize {
        self.slice.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.slice.is_empty()
    }

    #[inline]
    fn empty(&mut self) {
        self.slice = &[];
    }

    #[inline]
    fn truncate(&mut self, len: usize) -> Result<()> {
        if self.slice.len() < len {
            Err(Error::UnexpectedEof)
        } else {
            self.slice = &self.slice[..len];
            Ok(())
        }
    }

    #[inline]
    fn offset_from(&self, base: &Self) -> usize {
        self.offset_from(*base)
    }

    #[inline]
    fn find(&self, byte: u8) -> Result<usize> {
        self.find(byte).ok_or(Error::UnexpectedEof)
    }

    #[inline]
    fn skip(&mut self, len: usize) -> Result<()> {
        if self.slice.len() < len {
            Err(Error::UnexpectedEof)
        } else {
            self.slice = &self.slice[len..];
            Ok(())
        }
    }

    #[inline]
    fn split(&mut self, len: usize) -> Result<Self> {
        let slice = self.read_slice(len)?;
        Ok(EndianSlice::new(slice, self.endian))
    }

    #[inline]
    fn to_slice(&self) -> Result<Cow<[u8]>> {
        Ok(self.slice.into())
    }

    #[inline]
    fn to_string(&self) -> Result<Cow<str>> {
        match str::from_utf8(self.slice) {
            Ok(s) => Ok(s.into()),
            _ => Err(Error::BadUtf8),
        }
    }

    #[inline]
    fn to_string_lossy(&self) -> Result<Cow<str>> {
        Ok(String::from_utf8_lossy(self.slice))
    }

    #[inline]
    fn read_u8_array<A>(&mut self) -> Result<A>
    where
        A: Sized + Default + AsMut<[u8]>,
    {
        let len = mem::size_of::<A>();
        let slice = self.read_slice(len)?;
        let mut val = Default::default();
        <A as AsMut<[u8]>>::as_mut(&mut val).clone_from_slice(slice);
        Ok(val)
    }

    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let slice = self.read_slice(1)?;
        Ok(slice[0])
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8> {
        let slice = self.read_slice(1)?;
        Ok(slice[0] as i8)
    }

    #[inline]
    fn read_u16(&mut self) -> Result<u16> {
        let slice = self.read_slice(2)?;
        Ok(self.endian.read_u16(slice))
    }

    #[inline]
    fn read_i16(&mut self) -> Result<i16> {
        let slice = self.read_slice(2)?;
        Ok(self.endian.read_i16(slice))
    }

    #[inline]
    fn read_u32(&mut self) -> Result<u32> {
        let slice = self.read_slice(4)?;
        Ok(self.endian.read_u32(slice))
    }

    #[inline]
    fn read_i32(&mut self) -> Result<i32> {
        let slice = self.read_slice(4)?;
        Ok(self.endian.read_i32(slice))
    }

    #[inline]
    fn read_u64(&mut self) -> Result<u64> {
        let slice = self.read_slice(8)?;
        Ok(self.endian.read_u64(slice))
    }

    #[inline]
    fn read_i64(&mut self) -> Result<i64> {
        let slice = self.read_slice(8)?;
        Ok(self.endian.read_i64(slice))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use endianity::NativeEndian;

    #[test]
    fn test_endian_slice_split_at() {
        let endian = NativeEndian;
        let slice = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let eb = EndianSlice::new(slice, endian);
        assert_eq!(
            eb.split_at(3),
            (
                EndianSlice::new(&slice[..3], endian),
                EndianSlice::new(&slice[3..], endian)
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_endian_slice_split_at_out_of_bounds() {
        let slice = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let eb = EndianSlice::new(slice, NativeEndian);
        eb.split_at(30);
    }
}
