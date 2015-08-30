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

pub trait IndexDatum {
    fn index_datum_type() -> IndexDatumType;
}

impl IndexDatum for u8 {
    fn index_datum_type() -> IndexDatumType { IndexDatumType::UnsignedByte }
}

impl IndexDatum for u16 {
    fn index_datum_type() -> IndexDatumType { IndexDatumType::UnsignedShort }
}
