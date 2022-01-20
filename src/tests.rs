use crate::{Grid1D, GridError};

#[test]
fn grid_one_cell() {
    let grid = Grid1D::new(1, 1, &[-0.5, 0., 0.5, 1., 1.5]).unwrap();
    assert_eq!(grid.span(), 1.);
}

#[test]
fn grid_one_cell_with_gc() {
    let grid = Grid1D::new(1, 2, &[0., 1., 2., 3., 4., 5., 6.]).unwrap();
    assert_eq!(grid.span(), 2.);
}

#[test]
fn grid_singular() {
    let grid = Grid1D::new(0, 0, &[]);
    assert!(matches!(grid, Err(GridError::SingularGrid)));
}

#[test]
fn grid_no_gc_lower() {
    let grid = Grid1D::new(1, 0, &[0., 0.5, 1., 1.5]);
    assert!(matches!(grid, Err(GridError::MissingPositions)));
}

#[test]
fn grid_no_gc_upper() {
    let grid = Grid1D::new(1, 1, &[-0.5, 0., 0.5, 1.]);
    assert!(matches!(grid, Err(GridError::MissingPositions)));
}

#[test]
fn grid_not_enough_points() {
    let grid = Grid1D::new(2, 1, &[-0.5, 0., 0.5, 1., 1.5]);
    assert!(matches!(grid, Err(GridError::MissingPositions)));
}

#[test]
fn grid_non_monotonic() {
    let grid = Grid1D::new(1, 1, &[0., 1., 0.5, 2., 3.]);
    assert!(matches!(grid, Err(GridError::NonMonotonic)));
}
