use leak_slice::LeakSliceExt;
use std::mem::forget;

#[test]
fn leak() {
    let slice = &mut [1, 3, 3, 7][..];
    let ptr = slice.leak();

    // forget about the slice
    forget(slice);

    // SAFETY: we forgot about the slice
    unsafe {
        assert_eq!(ptr.as_ref(), [1, 3, 3, 7]);
    }
}
