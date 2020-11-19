#![no_implicit_prelude]
#![no_std]
#![feature(optin_builtin_traits)]
#![feature(negative_impls)]

mod std;

/// A mutable memory location.
#[derive(Default)]
#[repr(transparent)]
pub struct ClosureCell<T>
where
    T: ?std::Sized,
{
    value: std::UnsafeCell<T>,
}

pub unsafe auto trait ClosureCellSafe {}

impl<T> !ClosureCellSafe for ClosureCell<T> where T: ?std::Sized {}

impl<T> ClosureCell<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            value: std::UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T> ClosureCell<T>
where
    T: ?std::Sized,
{
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }
}

impl<T> ClosureCell<T>
where
    T: ?std::Sized + ClosureCellSafe,
{
    #[inline]
    pub fn with_inner(&self, f: impl std::FnOnce(&mut T) + ClosureCellSafe) {
        f(unsafe { &mut *self.value.get() });
    }
}

impl<T: std::Copy> ClosureCell<T> {
    /// Returns a copy of the contained value.
    #[inline]
    pub fn get(&self) -> T {
        // SAFETY: This can cause data races if called from a separate thread,
        // but `Cell` is `!Sync` so this won't happen.
        unsafe { *self.value.get() }
    }
}

impl<T> std::From<T> for ClosureCell<T> {
    #[inline]
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T: std::Copy> std::Clone for ClosureCell<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.get())
    }
}
