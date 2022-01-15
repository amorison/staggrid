use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use staggrid::{Grid1D, GridError};

create_exception!(staggrid, StaggridError, PyException);
create_exception!(staggrid, SingularGridError, StaggridError);
create_exception!(staggrid, NonMonotonicGridError, StaggridError);

trait IntoPyErr {
    fn into_py_err(self) -> PyErr;
}

impl IntoPyErr for GridError {
    fn into_py_err(self) -> PyErr {
        match self {
            GridError::SingularGrid =>
                SingularGridError::new_err(self.to_string()),
            GridError::NonMonotonic =>
                NonMonotonicGridError::new_err(self.to_string()),
        }
    }
}

trait IntoPyResult<T> {
    fn into_py_result(self) -> PyResult<T>;
}

impl<T, E> IntoPyResult<T> for Result<T, E>
where E: IntoPyErr
{
    fn into_py_result(self) -> PyResult<T> {
        self.map_err(IntoPyErr::into_py_err)
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
        let grid = Grid1D::from_slice(vals).into_py_result()?;
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
