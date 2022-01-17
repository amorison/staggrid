#![deny(unsafe_op_in_unsafe_fn)]

use ndarray::{Array1,ArrayView1};
use thiserror::Error;

pub struct Grid1D(Array1<f64>);

#[derive(Error, Debug)]
pub enum GridError {
    #[error("Grids must span over at least 2 points.")]
    SingularGrid,
    #[error("Values must increase in a monotonic fashion.")]
    NonMonotonic,
}

impl Grid1D {
    pub fn from_array(array: Array1<f64>) -> Result<Self, GridError> {
        if array.len() < 2 {
            return Err(GridError::SingularGrid)
        }
        if array.windows(2).into_iter()
            .any(|view| view[0] >= view[1])
        {
            return Err(GridError::NonMonotonic)
        };
        Ok(Grid1D(array))
    }

    pub fn from_slice(slc: &[f64]) -> Result<Self, GridError> {
        let view = ArrayView1::from(slc);
        Grid1D::from_array(view.to_owned())
    }

    pub fn first(&self) -> f64 {
        *self.0.first().unwrap()
    }

    pub fn last(&self) -> f64 {
        *self.0.last().unwrap()
    }

    pub fn span(&self) -> f64 {
        self.last() - self.first()
    }

    pub fn contains(&self, x: f64) -> bool {
        x >= self.first() && x <= self.last()
    }
}

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("First bound should be smaller than last one.")]
    InvalidBounds,
}

#[derive(Copy, Clone, Debug)]
struct Bounds(f64, f64);

impl TryFrom<(f64, f64)> for Bounds {
    type Error = DomainError;

    fn try_from(value: (f64, f64)) -> Result<Self, Self::Error> {
        let (first, last) = value;
        if first < last {
            Ok(Bounds(first, last))
        } else {
            Err(DomainError::InvalidBounds)
        }
    }
}

impl Bounds {
    fn is_in_grid(&self, grid: &Grid1D) -> bool {
        grid.contains(self.0) && grid.contains(self.1)
    }
}

pub struct Domain<const NDIM: usize> {
    boundaries: [Bounds; NDIM],
}

impl<const NDIM: usize> Domain<NDIM> {
    pub fn new(bounds: [(f64, f64); NDIM]) -> Result<Domain<NDIM>, DomainError> {
        let mut boundaries: Vec<Bounds> = Vec::with_capacity(NDIM);
        for b in bounds.into_iter() {
            boundaries.push(b.try_into()?);
        }
        let boundaries: [Bounds; NDIM] = boundaries.try_into().unwrap();
        Ok(Domain { boundaries })
    }
}

#[cfg(test)]
mod tests;
