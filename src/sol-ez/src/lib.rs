use std::marker::PhantomData;

pub struct ReadOnly;
pub struct Mutable;
pub struct Signed;
pub struct Unsigned;

pub trait Read {}
pub trait Write {}

impl Read for ReadOnly {}
impl Read for Mutable {}

impl Write for Mutable {}

pub struct Account<T, M, S> {
    inner: T,
    _mutable: PhantomData<M>,
    _signed: PhantomData<S>,
}

impl<T, M, S> Account<T, M, S> {
    pub fn as_ref(&self) -> &T
    where
        M: Read,
    {
        &self.inner
    }

    pub fn as_ref_mut(&mut self) -> &mut T
    where
        M: Write,
    {
        &mut self.inner
    }
}
