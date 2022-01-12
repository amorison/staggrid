use crate::Grid1D;

#[no_mangle]
pub unsafe extern "C" fn grid_c_from_slice(data: *const f64, len: usize)
    -> *mut Grid1D
{
    let slc = unsafe { std::slice::from_raw_parts(data, len) };
    match Grid1D::from_slice(slc) {
        Ok(grid) => {
            let g = Box::new(grid);
            Box::into_raw(g)
        },
        Err(_) => std::ptr::null_mut()
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
