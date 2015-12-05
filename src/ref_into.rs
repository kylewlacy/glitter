pub trait RefInto<'a, T>: MutInto<'a, T> {
    fn ref_into(&'a self) -> T;
}

pub trait MutInto<'a, T> {
    fn mut_into(&'a mut self) -> T;
}



impl<'a, T: 'a> RefInto<'a, &'a T> for T {
    fn ref_into(&'a self) -> &'a T {
        self
    }
}

impl<'a, T: 'a> RefInto<'a, &'a T> for &'a T {
    fn ref_into(&'a self) -> &'a T {
        &**self
    }
}

impl<'a, T: 'a> MutInto<'a, &'a T> for T {
    fn mut_into(&'a mut self) -> &'a T {
        self
    }
}

impl<'a, T: 'a> MutInto<'a, &'a T> for &'a T {
    fn mut_into(&'a mut self) -> &'a T {
        &**self
    }
}

impl<'a, T: 'a> MutInto<'a, &'a T> for &'a mut T {
    fn mut_into(&'a mut self) -> &'a T {
        &**self
    }
}

impl<'a, T: 'a> MutInto<'a, &'a mut T> for T {
    fn mut_into(&'a mut self) -> &'a mut T {
        self
    }
}

impl<'a, T: 'a> MutInto<'a, &'a mut T> for &'a mut T {
    fn mut_into(&'a mut self) -> &'a mut T {
        &mut **self
    }
}
