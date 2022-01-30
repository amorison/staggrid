#![deny(unsafe_op_in_unsafe_fn)]

use ndarray::ArrayView1;
use staggrid::{Grid1D, Position};

/// C API equivalent of [`Position::Walls`]
#[no_mangle]
pub static POSITION_WALLS: u8 = 0;

/// C API equivalent of [`Position::Centers`]
#[no_mangle]
pub static POSITION_CENTERS: u8 = 1;

/// Obtain a raw pointer to a [`Grid1D`] object.  It is the C API equivalent
/// of [`Grid1D::new`].
///
/// # Memory
///
/// It is the caller responsibility to drop the [`Grid1D`] object using
/// the [`grid_c_destroy`] function.
///
///
/// # Safety
///
/// It is the caller responsibility to pass non-null and aligned pointers
/// as arguments.  Additionally, `positions` should be valid for at least
/// `len_positions` reads.
///
/// This function returns a null pointer in case of error.  `ierr` contains
/// the corresponding error code.
///
/// The returned pointer should be considered opaque and not be inspected by
/// the caller.  Use the functions exposed in this crate to interact with the
/// underlying [`Grid1D`] object.
#[no_mangle]
pub unsafe extern "C" fn grid_c_create(
    nbulk_cells: usize, ilower_wall: usize,
    positions: *const f64, len_positions: usize,
    ierr: *mut i32,
    ) -> *mut Grid1D
{
    let slc = unsafe { std::slice::from_raw_parts(positions, len_positions) };
    match Grid1D::new(nbulk_cells, ilower_wall, slc) {
        Ok(grid) => {
            unsafe { *ierr = 0 };
            let g = Box::new(grid);
            Box::into_raw(g)
        },
        Err(e) => {
            unsafe { *ierr = e as i32 + 1 };
            std::ptr::null_mut()
        }
    }
}

/// Deallocate a [`Grid1D`] object, typically previously obtained by calling
/// [`grid_c_create`].
///
/// # Safety
///
/// It is the caller responsibility to pass a non-null, aligned, and
/// initialized pointer to a [`Grid1D`] object.  Pointers obtained with
/// [`grid_c_create`] are valid.  After calling this function, `grid` is a
/// dangling pointer.
#[no_mangle]
pub unsafe extern "C" fn grid_c_destroy(grid: *mut Grid1D) {
    drop(unsafe { Box::from_raw(grid) });
}

/// Return the span of the [`Grid1D`] object, excluding ghost cells.  This is
/// the C API equivalent of [`Grid1D::span`].
///
/// # Safety
///
/// `grid` should be a non-null, aligned, and initialized pointer.  Such
/// a pointer is typically obtained via [`grid_c_create`].
#[no_mangle]
pub unsafe extern "C" fn grid_c_span(grid: *mut Grid1D) -> f64 {
    let grid = unsafe{ &*grid };
    grid.span()
}

fn position_from_int(int: u8) -> Option<Position> {
    match int {
        0 => Some(Position::Walls),
        1 => Some(Position::Centers),
        _ => None,
    }
}

/// Produce a raw slice pointing to a copy of the elements in an [`ArrayView1`]
fn copy_view_into_c_slice(arr: &ArrayView1<f64>) -> Option<(*mut f64, usize)> {
    let len = arr.len();
    if len == 0 {
        // This is to always return None when the array is empty as malloc(0)
        // behaviour is implementation defined and return either a null pointer
        // (which would cause this function to return None) or a valid pointer
        // (which would cause this function to return it).
        return None
    }
    let ptr = unsafe {
        // SAFETY: the size won't overflow an usize as arr is backed by
        // a Vec which itself won't allocate more than isize::MAX bytes.
        // Moreover, the implementation defined case of `malloc(0)` cannot
        // happen as the `len == 0` case has already been handled.
        libc::malloc(std::mem::size_of::<f64>() * len)
    } as *mut f64;
    if ptr.is_null() {
        return None
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
    Some((ptr, len))
}

/// Return the grid points at a given position.  See [`POSITION_WALLS`]
/// and [`POSITION_CENTERS`].  This is the C API equivalent of [`Grid1D::at`].
/// The positions are returned as a contiguous slice of memory of size
/// `length`.
///
/// # Memory
///
/// The pointer returned by this function points to a `malloc`-allocated
/// chunk of memory.  The caller is responsible for `free`-ing the memory
/// behind this pointer.
///
/// # Safety
///
/// `grid` should be a non-null, aligned, and initialized pointer.  Such
/// a pointer is typically obtained via [`grid_c_create`].  Other pointers
/// should be non-null and aligned.
///
/// The fonction returns a null-pointer and `length` is set to `0` in case
/// of error. `ierr` is set to `1` if an invalid `position` was requested,
/// and `-1` if `malloc` failed.
#[no_mangle]
pub unsafe extern "C" fn grid_c_at(
    grid: *mut Grid1D,
    position: u8,
    length: *mut usize,
    ierr: *mut i32,
    ) -> *mut f64
{
    unsafe { *length = 0 };
    let position = match position_from_int(position) {
        Some(p) => p,
        None => {
            unsafe { *ierr = 1 };
            return std::ptr::null_mut()
        },
    };

    let grid = unsafe { &*grid };
    let values = grid.at(position);

    match copy_view_into_c_slice(&values) {
        None => {
            unsafe { *ierr = -1 };
            std::ptr::null_mut()
        },
        Some((ptr, len)) => {
            unsafe { *length = len };
            unsafe { *ierr = 0 };
            ptr
        }
    }
}
