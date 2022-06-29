use std::ptr::NonNull;

/// `LeakSliceExt` - leak slice in favor of `NonNull`
pub trait LeakSliceExt<T> {
    /// leak `&mut [T]` in favor of `NonNull<[T]>`
    fn leak(self) -> NonNull<[T]>;
}

impl<T> LeakSliceExt<T> for &mut [T] {
    fn leak(self) -> NonNull<[T]> {
        NonNull::from(self)
    }
}
