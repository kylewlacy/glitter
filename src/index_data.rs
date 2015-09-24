pub use std::mem;
pub use std::slice;

pub enum IndexDatumType {
    UnsignedByte,
    UnsignedShort
}

pub trait IndexData {
    fn index_datum_type() -> IndexDatumType;
    fn index_bytes(&self) -> &[u8];
    fn index_elements(&self) -> usize;
}

pub unsafe trait IndexDatum {
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
