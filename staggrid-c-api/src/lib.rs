#![deny(unsafe_op_in_unsafe_fn)]

use staggrid::Grid1D;

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
