pub trait RefInto<'a, T>: MutInto<'a, T> {
    fn ref_into(&'a self) -> T;
}

pub trait MutInto<'a, T> {
    fn mut_into(&'a mut self) -> T;
}
