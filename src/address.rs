use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct Address<T> {
    pub val: u64,
    _marker: PhantomData<*const T>,
}

impl<T> Copy for Address<T> { }

impl<T> Clone for Address<T> {
    fn clone(&self) -> Address<T> {
        *self
    }
}

impl<T> fmt::Debug for Address<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Addr:({}){}", std::any::type_name::<T>(), self.val)
    }
}

impl<T> fmt::Display for Address<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<T> From<u64> for Address<T> {
    fn from(val: u64) -> Address<T> {
        Address::<T> {
            val: val,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Address<T> {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T> DerefMut for Address<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}
