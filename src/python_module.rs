use numpy::PyArray1;
use pyo3::{exceptions::PyValueError, prelude::*};
use crate::{Grid1D, GridError};

impl std::convert::From<GridError> for PyErr {
    fn from(err: GridError) -> Self {
        // should try to mirror the existing errors
        PyValueError::new_err(err.to_string())
    }
}

#[pyclass(name="Grid1D")]
struct Grid1Dpy(Grid1D);

#[pymethods]
impl Grid1Dpy {
    #[new]
    fn new(vals: &PyArray1<f64>) -> PyResult<Self> {
        let vals = vals.readonly();
        let vals = vals.as_slice()?;
        let grid = Grid1D::from_slice(vals)?;
        Ok(Grid1Dpy(grid))
    }

    fn span(&self) -> f64 {
        self.0.span()
    }
}

#[pymodule]
fn staggrid(_py: Python<'_>, pymod: &PyModule) -> PyResult<()> {
    pymod.add_class::<Grid1Dpy>()?;
    Ok(())
}
