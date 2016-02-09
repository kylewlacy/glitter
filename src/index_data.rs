//! Contains types related to index data, which are used for [`IndexBuffers`]
//! (../vertex_buffer/struct.IndexBuffer.html).

pub use std::mem;
pub use std::slice;

/// The set of possible [`IndexDatum`](trait.IndexDatum.html) types.
pub enum IndexDatumType {
    /// Unsigned 8-bit index data.
    UnsignedByte,

    /// Unsigned 16-bit index data.
    UnsignedShort
}

/// Indicates that a type can be coerced to a `u8` slice that can
/// be treated as a stream of index data.
pub trait IndexData {
    /// Return the data type that this index data should be treated as.
    fn index_datum_type() -> IndexDatumType;

    /// Create a byte slice of index data from `self`.
    fn index_bytes(&self) -> &[u8];

    /// Return the number of index elements that `self` contains.
    fn index_elements(&self) -> usize;
}

/// A single value that can be treated as a single index.
///
/// # Safety
/// This type will be transmuted to a slice according to the value
/// returned by the [`index_datum_type`]
/// (trait.IndexDatum.html#tymethod.index_datum_type) method. Implementing
/// this method incorrectly will lead to memory unsafety.
pub unsafe trait IndexDatum {
    /// Return the data type this datum corresponds to.
    ///
    /// # Safety
    /// An instance of this type must match the size and memory layout
    /// specified by the returned [`IndexDatumType`](enum.IndexDatumType.html).
    fn index_datum_type() -> IndexDatumType;
}

unsafe impl IndexDatum for u8 {
    fn index_datum_type() -> IndexDatumType { IndexDatumType::UnsignedByte }
}

unsafe impl IndexDatum for u16 {
    fn index_datum_type() -> IndexDatumType { IndexDatumType::UnsignedShort }
}

impl<T: IndexDatum> IndexData for [T] {
    fn index_datum_type() -> IndexDatumType {
        T::index_datum_type()
    }

    fn index_bytes(&self) -> &[u8] {
        let size = mem::size_of::<T>() * self.len();
        unsafe {
            slice::from_raw_parts(mem::transmute(&self[0]), size)
        }
    }

    fn index_elements(&self) -> usize {
        self.len()
    }
}
