# Leak `&mut [T]` in favor of `NonNull<[T]>`
```rust
use leak_slice::LeakSliceExt;
use std::mem::forget;

fn main() {
    let slice = &mut [1, 3, 3, 7][..];
    let ptr = slice.leak();

    // forget about the slice
    forget(slice); // optional, but still don't use slice

    // SAFETY: we forgot about the slice
    unsafe {
        assert_eq!(ptr.as_ref(), [1, 3, 3, 7]);
    }
}
```