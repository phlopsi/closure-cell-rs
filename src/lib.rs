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
    /// Creates a new `ClosureCell` containing the given value.
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            value: std::UnsafeCell::new(value),
        }
    }

    /// Sets the contained value.
    #[inline]
    pub fn set(&self, value: T) {
        let old = self.replace(value);
        std::mem::drop(old);
    }

    /// Swaps the values of two Cells.
    /// Difference with `std::mem::swap` is that this function doesn't require `&mut` reference.
    #[inline]
    pub fn swap(&self, other: &Self) {
        if std::ptr::eq(self, other) {
            return;
        }

        // SAFETY: This can be risky if called from separate threads, but `Cell`
        // is `!Sync` so this won't happen. This also won't invalidate any
        // pointers since `ClosureCell` makes sure nothing else will be pointing into
        // either of these `ClosureCell`s.
        unsafe {
            std::ptr::swap(self.value.get(), other.value.get());
        }
    }

    /// Replaces the contained value, and returns it.
    pub fn replace(&self, value: T) -> T {
        // SAFETY: This can cause data races if called from a separate thread,
        // but `ClosureCell` is `!Sync` so this won't happen.
        std::mem::replace(unsafe { &mut *self.value.get() }, value)
    }

    /// Unwraps the value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T> ClosureCell<T>
where
    T: ?std::Sized,
{
    /// Returns a raw pointer to the underlying data in this cell.
    #[inline]
    pub const fn as_ptr(&self) -> *mut T {
        self.value.get()
    }

    /// Returns a mutable reference to the underlying data.
    ///
    /// This call borrows `ClosureCell` mutably (at compile-time) which guarantees
    /// that we possess the only reference.
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }

    /// Returns a `&Cell<T>` from a `&mut T`
    #[inline]
    pub fn from_mut(ref_mut: &mut T) -> &Self {
        // SAFETY: `&mut` ensures unique access.
        unsafe { &*(ref_mut as *mut T as *const Self) }
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
