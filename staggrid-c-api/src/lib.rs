#![deny(unsafe_op_in_unsafe_fn)]

use staggrid::Grid1D;

pub mod raw_slice;

use raw_slice::RawSlice;

/// C API equivalent of [`staggrid::Position`]
#[repr(C)]
pub enum Position {
    Walls,
    Centers,
}

impl From<Position> for staggrid::Position {
    fn from(pos: Position) -> Self {
        match pos {
            Position::Walls => staggrid::Position::Walls,
            Position::Centers => staggrid::Position::Centers,
        }
    }
}

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

/// Return the grid points at a given [`Position`].  This is the C API
/// equivalent of [`Grid1D::at`].
///
/// # Safety
///
/// `grid` should be a non-null, aligned, and initialized pointer.  Such
/// a pointer is typically obtained via [`grid_c_create`].  `ierr` should be
/// non-null and aligned.
///
/// See [`RawSlice`] documentation for safety and memory considerations.
#[no_mangle]
pub unsafe extern "C" fn grid_c_at(
    grid: *mut Grid1D,
    position: Position,
    ) -> RawSlice
{
    let grid = unsafe { &*grid };
    let values = grid.at(position.into());
    values.into()
}
