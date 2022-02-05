#![deny(unsafe_op_in_unsafe_fn)]

use staggrid::{Grid1D, Position};

pub mod raw_slice;

use raw_slice::RawSlice;

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

/// Return the grid points at a given position.  See [`POSITION_WALLS`]
/// and [`POSITION_CENTERS`].  This is the C API equivalent of [`Grid1D::at`].
///
/// `ierr` is set to `1` if an invalid `position` was requested,
/// and `-1` if `malloc` failed.
///
/// # Safety
///
/// `grid` should be a non-null, aligned, and initialized pointer.  Such
/// a pointer is typically obtained via [`grid_c_create`].  `ierr` should be
/// non-null and aligned.
///
/// See [`raw_slice`] for safety and memory considerations about [`RawSlice`].
#[no_mangle]
pub unsafe extern "C" fn grid_c_at(
    grid: *mut Grid1D,
    position: u8,
    ierr: *mut i32,
    ) -> RawSlice
{
    let position = match position_from_int(position) {
        Some(p) => p,
        None => {
            unsafe { *ierr = 1 };
            return RawSlice::empty();
        },
    };

    let grid = unsafe { &*grid };
    let values = grid.at(position);

    let slc: RawSlice = values.into();
    if slc.is_empty() {
        unsafe { *ierr = -1 };
    } else {
        unsafe { *ierr = 0 };
    }

    slc
}
