#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_assignments)]
pub mod utils_rust;
pub mod spacetime;
pub mod groove;
pub mod relaxed_ik;
pub mod relaxed_ik_wrapper;
pub mod relaxed_ik_web;

use pyo3::prelude::*;
use pyo3::types::PyTuple;
use numpy::{PyArray1, ToPyArray};
use nalgebra::{Vector3, UnitQuaternion, Quaternion};

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
    fn get_current_goal<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let position = self.inner.vars.goal_positions[0];
        let quaternion = self.inner.vars.goal_quats[0];

        let position_array = PyArray1::from_vec_bound(
            py, vec![position.x, position.y, position.z]);
        let quaternion_array = PyArray1::from_vec_bound(
            py, vec![quaternion.w, quaternion.i, quaternion.j, quaternion.k]);

        Ok(PyTuple::new_bound(py, &[position_array, quaternion_array]))
    }

    pub fn solve<'py>(&mut self,
                      py: Python<'py>,
                      position: &PyArray1<f64>,
                      quaternion: &PyArray1<f64>) -> PyResult<Bound<'py,PyArray1<f64>>> {
        let pos_slice = unsafe { position.as_slice().unwrap() };
        let quat_slice = unsafe { quaternion.as_slice().unwrap() };

        self.inner.vars.goal_positions[0] = Vector3::new(
            pos_slice[0], pos_slice[1], pos_slice[2]);
        self.inner.vars.goal_quats[0] = UnitQuaternion::from_quaternion(
            Quaternion::new(quat_slice[0], quat_slice[1], quat_slice[2], quat_slice[3]));
        
        let result = self.inner.solve();
        Ok(result.to_pyarray_bound(py))
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
