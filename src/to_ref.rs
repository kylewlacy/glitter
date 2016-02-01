pub trait ToRef<'a> {
    type Ref: 'a;

    fn to_ref(&'a self) -> Self::Ref;
}

pub trait ToMut<'a>: ToRef<'a> {
    type Mut: 'a;

    fn to_mut(&'a mut self) -> Self::Mut;
}
