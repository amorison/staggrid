#![deny(unsafe_op_in_unsafe_fn)]

use libc;
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
    let err_code = unsafe { ierr.as_mut() }.unwrap();
    match Grid1D::new(nbulk_cells, ilower_wall, slc) {
        Ok(grid) => {
            *err_code = 0;
            let g = Box::new(grid);
            Box::into_raw(g)
        },
        Err(e) => {
            *err_code = e as i32 + 1;
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
    let grid = unsafe { Box::from_raw(grid) };
    let span = grid.span();
    Box::leak(grid);
    span
}

fn position_from_int(int: u8) -> Option<Position> {
    match int {
        0 => Some(Position::Walls),
        1 => Some(Position::Centers),
        _ => None,
    }
}

#[no_mangle]
pub unsafe extern "C" fn grid_c_at(
    grid: *mut Grid1D,
    position: u8,
    length: *mut usize,
    ierr: *mut i32,
    ) -> *mut f64
{
    let grid = unsafe { Box::from_raw(grid) };
    unsafe { *length = 0 };
    let position = match position_from_int(position) {
        Some(p) => p,
        None => {
            unsafe { *ierr = 1 };
            return std::ptr::null_mut()
        },
    };

    let values = grid.at(position);

    let ptr = unsafe {
        libc::malloc(std::mem::size_of::<f64>() * values.len())
    } as *mut f64;
    if ptr.is_null() {
        unsafe { *ierr = -1 };
        return ptr
    }
    for (i, &val) in values.iter().enumerate() {
        unsafe { *ptr.add(i) = val };
    }
    unsafe { *length = values.len() };
    unsafe { *ierr = 0 };
    Box::leak(grid);
    ptr
}
