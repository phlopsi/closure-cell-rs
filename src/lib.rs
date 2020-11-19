#![feature(optin_builtin_traits)]
#![feature(negative_impls)]

mod std;

pub struct ClosureCell<T>
where
    T: ?std::Sized,
{
    value: std::UnsafeCell<T>,
}

pub auto trait ClosureCellSafe {}

impl<T> !ClosureCellSafe for ClosureCell<T> where T: ?std::Sized {}

impl<T> ClosureCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: std::UnsafeCell::new(value),
        }
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
