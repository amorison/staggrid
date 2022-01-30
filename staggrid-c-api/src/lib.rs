#![deny(unsafe_op_in_unsafe_fn)]

use ndarray::ArrayView1;
use staggrid::{Grid1D, Position};

#[no_mangle]
pub static POSITION_WALLS: u8 = 0;
#[no_mangle]
pub static POSITION_CENTERS: u8 = 1;

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

#[no_mangle]
pub unsafe extern "C" fn grid_c_destroy(grid: *mut Grid1D) {
    drop(unsafe { Box::from_raw(grid) });
}

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
        libc::malloc(std::mem::size_of::<f64>() * len)
    } as *mut f64;
    if ptr.is_null() {
        return None
    }
    if arr.is_standard_layout() {
        let src = arr.as_ptr();
        unsafe {
            ptr.copy_from_nonoverlapping(src, len);
        }
    } else {
        for (i, &val) in arr.iter().enumerate() {
            unsafe { *ptr.add(i) = val };
        }
    }
    Some((ptr, len))
}

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
