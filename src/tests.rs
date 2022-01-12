use ndarray::Array1;
use crate::{Grid1D, GridError};

#[test]
fn grid_from_array() {
    let arr = Array1::linspace(0., 1., 5);
    let grid = Grid1D::from_array(arr).unwrap();
    assert_eq!(grid.span(), 1.);
}

#[test]
fn grid_from_slice() {
    let grid = Grid1D::from_slice(&[0., 0.5, 1.]).unwrap();
    assert_eq!(grid.span(), 1.);
}

#[test]
fn grid_singular() {
    let grid = Grid1D::from_slice(&[0.]);
    assert!(matches!(grid, Err(GridError::SingularGrid)));
}

#[test]
fn grid_non_monotonic() {
    let grid = Grid1D::from_slice(&[0., 1., 0.5]);
    assert!(matches!(grid, Err(GridError::NonMonotonic)));
}
