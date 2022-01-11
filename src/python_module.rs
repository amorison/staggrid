use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use crate::{Grid1D, GridError};

create_exception!(staggrid, StaggridError, PyException);
create_exception!(staggrid, SingularGridError, StaggridError);
create_exception!(staggrid, NonMonotonicGridError, StaggridError);

impl std::convert::From<GridError> for PyErr {
    fn from(err: GridError) -> Self {
        match err {
            GridError::SingularGrid => {
                SingularGridError::new_err(err.to_string())
            },
            GridError::NonMonotonic => {
                NonMonotonicGridError::new_err(err.to_string())
            },
        }
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
fn staggrid(py: Python<'_>, pymod: &PyModule) -> PyResult<()> {
    pymod.add_class::<Grid1Dpy>()?;
    pymod.add("StaggridError", py.get_type::<StaggridError>())?;
    pymod.add("SingularGridError", py.get_type::<SingularGridError>())?;
    pymod.add("NonMonotonicGridError", py.get_type::<NonMonotonicGridError>())?;
    Ok(())
}
