use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::fmt;

pub struct Address<T> {
    val: u64,
    _marker: PhantomData<*const T>,
}

impl<T> fmt::Debug for Address<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Addr:({}){}", std::any::type_name::<T>(), self.val)
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
