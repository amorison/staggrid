use ndarray::{Array1, ArrayView1, Slice};
use thiserror::Error;

pub struct Grid1D {
    walls: Array1<f64>,
    centers: Array1<f64>,
    ilower_wall: usize,
    iupper_wall: usize,
    ilower_center: usize,
    iupper_center: usize,
}

pub enum Position {
    Walls,
    Centers,
}

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

    pub fn at(&self, position: Position) -> ArrayView1<f64> {
        match position {
            Position::Walls => self.walls.view(),
            Position::Centers => self.centers.view(),
        }
    }

    pub fn bulk_slice_of(&self, position: Position) -> Slice {
        match position {
            Position::Walls =>
                (self.ilower_wall..=self.iupper_wall).into(),
            Position::Centers =>
                (self.ilower_center..=self.iupper_center).into(),
        }
    }

    pub fn span(&self) -> f64 {
        self.walls[self.iupper_wall] - self.walls[self.ilower_wall]
    }
}

#[cfg(test)]
mod tests;
