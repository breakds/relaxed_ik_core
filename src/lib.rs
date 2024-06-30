pub mod utils_rust;
pub mod spacetime;
pub mod groove;
pub mod relaxed_ik;
pub mod relaxed_ik_wrapper;
pub mod relaxed_ik_web;

use pyo3::prelude::*;
use pyo3::types::PyTuple;
use numpy::{PyArray1, ToPyArray};

#[pyclass]
struct RelaxedIK {
    inner: relaxed_ik::RelaxedIK,
}

#[pymethods]
impl RelaxedIK {
    #[new]
    fn new(path_to_setting: &str) -> Self {
        RelaxedIK{inner: relaxed_ik::RelaxedIK::load_settings(path_to_setting)}
    }

    // Note that the lifetime annotation `'py` ensures that the returned tuple
    // is correctly associated with the Python interpreter's lifetime.
    
    #[getter]
    fn get_current_goal<'py>(&self, py: Python<'py>) -> PyResult<&'py PyTuple> {
        let position = self.inner.vars.goal_positions[0];
        let quaternion = self.inner.vars.goal_quats[0];

        let position_array = PyArray1::from_vec(py, vec![position.x, position.y, position.z]);
        let quaternion_array = PyArray1::from_vec(py, vec![quaternion.w, quaternion.i, quaternion.j, quaternion.k]);

        Ok(PyTuple::new(py, &[position_array, quaternion_array]))
    }    
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn relaxed_ik_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RelaxedIK>()?;
    Ok(())
}
