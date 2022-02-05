use std::mem::ManuallyDrop;

use ndarray::ArrayView1;

/// Raw contiguous slice for consumption by C caller.
///
/// It has two fields (accessible in C code, kept private on Rust side):
/// - `ptr: double*`, the pointer to the first element of the slice;
/// - `len: uintptr_t`, the length of the slice.
///
/// # Memory
///
/// The caller obtaining a [`RawSlice`] on the C-side is responsible for
/// freeing the memory the [`RawSlice`] points to.  This can be done with
/// the [`raw_slice_c_nullify`] function.  Note that dropping a [`RawSlice`]
/// on the Rust side _does_ deallocate the underlying array.
///
/// # Safety
///
/// Many functions exposed to C in this crate are faillible.  The receiver of a
/// [`RawSlice`] should therefore check that the pointer is not `NULL` before
/// attempting to use it.  Slices with a length of zero have a `NULL` pointer.
///
/// [`RawSlice`]s returned by the API are guaranteed to either:
/// - have a non-null, initialized `ptr` valid for `len` reads;
/// - have a null pointer with a `len` of `0`.
///
/// Calling code should never manually modify a [`RawSlice`] to avoid breaking
/// these guarantees and triggering Undefined Behaviour (e.g. when freeing
/// the memory with [`raw_slice_c_nullify`]).
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

    /// Deallocate existing slice, set its pointer to null and its length to 0.
    pub fn nullify(&mut self) {
        let other_slice = RawSlice { ptr: self.ptr, len: self.len };
        self.ptr = std::ptr::null_mut();
        self.len = 0;
        drop(other_slice);
    }
}

impl From<ArrayView1<'_, f64>> for RawSlice {
    fn from(arr: ArrayView1<f64>) -> Self {
        let len = arr.len();
        if len == 0 {
            return RawSlice::empty();
        }
        let slc = arr.to_vec().into_boxed_slice();
        let mut slc = ManuallyDrop::new(slc);
        let ptr = slc.as_mut_ptr();
        RawSlice { ptr, len }
    }
}

impl Drop for RawSlice {
    fn drop(&mut self) {
        if !self.is_empty() {
            unsafe {
                // SAFETY: this is safe as a RawSlice is always built such that
                // ptr is valid for len reads and writes.
                let slc = std::slice::from_raw_parts_mut(self.ptr, self.len);
                drop(Box::from_raw(slc));
            }
        }
    }
}

/// Deallocate the array behind a slice and set the passed slice to a null
/// `ptr` and `len` of `0`.  This is the C API equivalent of
/// [`RawSlice::nullify`].
///
/// # Safety
///
/// `rawslc` should be a non-null, aligned, and initialized pointer.  The
/// [`RawSlice`] pointed to should be valid, in particular its `ptr` and
/// `len` fields should not have been modified by calling C code.
#[no_mangle]
pub unsafe extern "C" fn raw_slice_c_nullify(rawslc: *mut RawSlice) {
    let slc = unsafe { &mut *rawslc };
    slc.nullify();
}
