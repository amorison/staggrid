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

    pub fn span(&self) -> f64 {
        self.0.last().unwrap() - self.0.first().unwrap()
    }
}

#[cfg(feature = "python-ffi")]
mod python_module;

mod c_ffi;
mod tests;
