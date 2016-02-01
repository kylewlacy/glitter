pub trait ToRef<'a> {
    type Ref: 'a;

    fn to_ref(&'a self) -> Self::Ref;
}

pub trait ToMut<'a>: ToRef<'a> {
    type Mut: 'a;

    fn to_mut(&'a mut self) -> Self::Mut;
}

impl<'a, T: 'a> ToRef<'a> for &'a T {
    type Ref = &'a T;

    fn to_ref(&'a self) -> Self::Ref {
        &*self
    }
}

impl<'a, T: 'a> ToRef<'a> for &'a mut T {
    type Ref = &'a T;

    fn to_ref(&'a self) -> Self::Ref {
        &*self
    }
}

impl<'a, T: 'a> ToMut<'a> for &'a mut T {
    type Mut = &'a mut T;

    fn to_mut(&'a mut self) -> Self::Mut {
        &mut *self
    }
}

impl<'a> ToRef<'a> for () {
    type Ref = ();

    fn to_ref(&'a self) -> Self::Ref {
        ()
    }
}

impl<'a> ToMut<'a> for () {
    type Mut = ();

    fn to_mut(&'a mut self) -> Self::Mut {
        ()
    }
}
