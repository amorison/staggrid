use ndarray::{Array1, ArrayView1, Slice};
use thiserror::Error;

/// This represents a 1D staggered grid with two families of points at
/// [`Position::Walls`] and [`Position::Centers`].
pub struct Grid1D {
    walls: Array1<f64>,
    centers: Array1<f64>,
    ilower_wall: usize,
    iupper_wall: usize,
    ilower_center: usize,
    iupper_center: usize,
}

/// The two families of points in a staggered grid.
pub enum Position {
    Walls,
    Centers,
}

/// Errors encountered when creating or manipulating a [`Grid1D`].
#[derive(Error, Debug)]
pub enum GridError {
    #[error("Grids must span over at least 1 cell.")]
    SingularGrid,
    #[error("Values must increase in a monotonic fashion.")]
    NonMonotonic,
    #[error("Not enough point positions provided.")]
    MissingPositions,
}

impl Grid1D {
    /// Create a [`Grid1D`] object.  `nbulk_cells` in the number of
    /// cells in the grid excluding ghosts cells.  `positions` are
    /// all the points (walls and centers) in the grid, in ascending
    /// order.  `ilower_wall` is the index of the wall at the lower
    /// boundary of the domain in the `positions` slice.
    pub fn new(
        nbulk_cells: usize, ilower_wall: usize, positions: &[f64]
    ) -> Result<Self, GridError>
    {
        if nbulk_cells == 0 {
            return Err(GridError::SingularGrid);
        }
        let iupper_wall = ilower_wall + 2 * nbulk_cells;
        if ilower_wall == 0 || iupper_wall >= positions.len() - 1 {
            // 0 and len - 1 here to ensure at least one center outside domain
            return Err(GridError::MissingPositions);
        }
        if positions.windows(2).any(|elts| elts[0] >= elts[1]) {
            return Err(GridError::NonMonotonic);
        }

        let walls = positions[(ilower_wall % 2)..].iter()
            .step_by(2).cloned().collect();
        let centers = positions[(1 - ilower_wall % 2)..].iter()
            .step_by(2).cloned().collect();
        let ilower_center = (ilower_wall + 1) / 2;
        let iupper_center = (iupper_wall - 1) / 2;
        let ilower_wall = ilower_wall / 2;
        let iupper_wall = iupper_wall / 2;
        Ok(Grid1D {
            walls, centers,
            ilower_wall, iupper_wall,
            ilower_center, iupper_center,
        })
    }

    /// Return the grid points at a given [`Position`]
    pub fn at(&self, position: Position) -> ArrayView1<f64> {
        match position {
            Position::Walls => self.walls.view(),
            Position::Centers => self.centers.view(),
        }
    }

    /// Return a [`Slice`] representing the part of the grid outside of the
    /// bulk of the domain (i.e. with ghost points excluded).
    pub fn bulk_slice_of(&self, position: Position) -> Slice {
        match position {
            Position::Walls =>
                (self.ilower_wall..=self.iupper_wall).into(),
            Position::Centers =>
                (self.ilower_center..=self.iupper_center).into(),
        }
    }

    /// Width of the physical domain of the grid
    pub fn span(&self) -> f64 {
        self.walls[self.iupper_wall] - self.walls[self.ilower_wall]
    }
}

#[cfg(test)]
mod tests;
