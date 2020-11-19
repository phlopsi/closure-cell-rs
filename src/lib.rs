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
    pub const fn new(value: T) -> Self {
        Self {
            value: std::UnsafeCell::new(value),
        }
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T> ClosureCell<T>
where
    T: ?std::Sized,
{
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }
}

impl<T> ClosureCell<T>
where
    T: ?std::Sized + ClosureCellSafe,
{
    pub fn with_inner(&self, f: impl std::FnOnce(&mut T) + ClosureCellSafe) {
        f(unsafe { &mut *self.value.get() });
    }
}

impl<T> std::From<T> for ClosureCell<T> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}
