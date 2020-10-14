use alloc::boxed::Box;
use core::{
    borrow::Borrow,
    cmp, fmt,
    hash::{Hash, Hasher},
    ops::Deref,
};

#[derive(Clone, Copy, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Wrapped<T: ?Sized>(T);

pub trait Wrap {
    fn wrap(&self) -> &Wrapped<Self>;
}

impl<T: ?Sized> Wrap for T {
    fn wrap(&self) -> &Wrapped<T> {
        let ptr = self as *const T as *const Wrapped<T>;
        unsafe { &*ptr }
    }
}

pub struct Semi<T> {
    ptr: *const T,
}

impl<T> Semi<T> {
    pub fn share(value: T) -> [Self; 2] {
        let ptr: *const T = Box::into_raw(Box::new(value));
        [Self { ptr }, Self { ptr }]
    }

    pub fn reunite(a: Self, b: Self) -> T {
        assert!(core::ptr::eq(a.ptr, b.ptr));
        unsafe { *Box::from_raw(a.ptr as *mut T) }
    }
}

impl<T> Deref for Semi<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<K: Borrow<Q>, Q: ?Sized> Borrow<Wrapped<Q>> for Semi<K> {
    fn borrow(&self) -> &Wrapped<Q> {
        K::borrow(self).wrap()
    }
}

impl<T: fmt::Debug> fmt::Debug for Semi<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        T::fmt(self, f)
    }
}

impl<T: Hash> Hash for Semi<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        T::hash(self, state);
    }
}

impl<T: Eq> Eq for Semi<T> {}

impl<T: Ord> Ord for Semi<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        T::cmp(self, other)
    }
}

impl<T: PartialEq> PartialEq for Semi<T> {
    fn eq(&self, other: &Self) -> bool {
        T::eq(self, other)
    }
}

impl<T: PartialOrd> PartialOrd for Semi<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        T::partial_cmp(self, other)
    }
}
