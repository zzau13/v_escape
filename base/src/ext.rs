#![allow(dead_code)]

// Adapted from https://github.com/BurntSushi/memchr/blob/master/src/ext.rs
/// A trait for adding some helper routines to pointers.
pub(crate) trait Pointer {
    /// Returns the distance, in units of `T`, between `self` and `origin`.
    ///
    /// # Safety
    ///
    /// Same as `ptr::offset_from` in addition to `self >= origin`.
    unsafe fn distance(self, origin: Self) -> usize;

    /// Casts this pointer to `usize`.
    ///
    /// Callers should not convert the `usize` back to a pointer if at all
    /// possible. (And if you believe it's necessary, open an issue to discuss
    /// why. Otherwise, it has the potential to violate pointer provenance.)
    /// The purpose of this function is just to be able to do arithmetic, i.e.,
    /// computing offsets or alignments.
    fn to_usize(self) -> usize;
}

impl<T> Pointer for *const T {
    unsafe fn distance(self, origin: *const T) -> usize {
        unsafe {
            // TODO: Replace with `ptr::sub_ptr` once stabilized.
            usize::try_from(self.offset_from(origin)).unwrap_unchecked()
        }
    }

    fn to_usize(self) -> usize {
        self as usize
    }
}
