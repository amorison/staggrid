use ndarray::ArrayView1;

/// Raw `malloc`-allocated contiguous slice for consumption by C caller.
///
/// It has two fields (accessible in C code, kept private on Rust side):
/// - `ptr: double*`, the pointer to the first element of the slice;
/// - `len: uintptr_t`, the length of the slice.
///
/// # Memory
///
/// By convention, arrays in this structure are `malloc`-allocated.  Conversion
/// from Rust types is done by copying.  The caller (typically on the C-side)
/// obtaining a [`RawSlice`] is responsible for `free()`-ing the memory the
/// [`RawSlice`] points to.
///
/// # Safety
///
/// Many functions exposed to C in this crate are faillible.  The receiver of a
/// [`RawSlice`] should therefore check that the pointer is not `NULL` before
/// attempting to use it.  Note that to avoid discrepencies depending on the
/// behaviour of `malloc(0)`, slices with a length of zero have a `NULL`
/// pointer.
///
/// [`RawSlice`]s returned by the API are guaranteed to either:
/// - have a non-null, initialized `ptr` valid for `len` reads;
/// - have a null pointer with a `len` of `0`.
///
/// Calling code should never modify a [`RawSlice`] to avoid breaking these
/// guarantees by mistake.
#[repr(C)]
pub struct RawSlice {
    ptr: *mut f64,
    len: usize,
}

impl RawSlice {
    /// Create an empty slice with a `NULL` pointer and a length of 0.
    pub fn empty() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
        }
    }

    /// Check whether slice has zero elements.
    pub fn is_empty(&self) -> bool {
        self.ptr.is_null()
    }
}

impl From<ArrayView1<'_, f64>> for RawSlice {
    fn from(arr: ArrayView1<f64>) -> Self {
        let len = arr.len();
        if len == 0 {
            // This is to always return None when the array is empty as
            // malloc(0) behaviour is implementation defined and return either
            // a null pointer (which would cause this function to return None)
            // or a valid pointer (which would cause this function to return
            // it).
            return RawSlice::empty();
        }
        let ptr = unsafe {
            // SAFETY: the size won't overflow an usize as arr is backed by a
            // Vec which itself won't allocate more than isize::MAX bytes.
            // Moreover, the implementation defined case of `malloc(0)` cannot
            // happen as the `len == 0` case has already been handled.
            libc::malloc(std::mem::size_of::<f64>() * len)
        } as *mut f64;
        if ptr.is_null() {
            return RawSlice::empty();
        }
        if arr.is_standard_layout() {
            let src = arr.as_ptr();
            unsafe {
                // SAFETY: ptr is know to be valid for len writes as this is the
                // size we allocated, and src is known to be valid for len reads as
                // arr wraps a contiguous slice of that length.
                ptr.copy_from_nonoverlapping(src, len);
            }
        } else {
            for (i, &val) in arr.iter().enumerate() {
                unsafe {
                    // SAFETY: we cannot simply memcpy as above since the
                    // elements are not contiguous.  We do know ptr is valid
                    // for as many writes as the number of elements in arr
                    // since this is what we allocated.
                    *ptr.add(i) = val
                };
            }
        }
        RawSlice { ptr, len }
    }
}
