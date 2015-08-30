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
