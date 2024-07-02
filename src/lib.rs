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
use numpy::{PyArray1, ToPyArray, PyReadonlyArray1};
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
        let p = self.inner.vars.goal_positions[0];
        let q = self.inner.vars.goal_quats[0];

        let p_array = PyArray1::from_slice_bound(py, &[p.x, p.y, p.z]);
        let q_array = PyArray1::from_slice_bound(py, &[q.w, q.i, q.j, q.k]);
        Ok(PyTuple::new_bound(py, &[p_array, q_array]))
    }

    pub fn solve<'py>(
        &mut self,
        py: Python<'py>,
        position: &PyArray1<f64>,
        quaternion: &PyArray1<f64>
    ) -> PyResult<(Bound<'py, PyArray1<f64>>, String)> {
        let pos_slice = unsafe { position.as_slice().unwrap() };
        let quat_slice = unsafe { quaternion.as_slice().unwrap() };

        self.inner.vars.goal_positions[0] = Vector3::new(
            pos_slice[0], pos_slice[1], pos_slice[2]);
        self.inner.vars.goal_quats[0] = UnitQuaternion::from_quaternion(
            Quaternion::new(quat_slice[0], quat_slice[1], quat_slice[2], quat_slice[3]));

        let (x, quality) = self.inner.solve();
        let quality_str = match quality {
            relaxed_ik::SolutionQuality::Failed => "Failed",
            relaxed_ik::SolutionQuality::NotConverged => "NotConverged",
            relaxed_ik::SolutionQuality::Success => "Success",
        }.to_string();
        Ok((x.to_pyarray_bound(py), quality_str))
    }

    pub fn forward<'py>(
        &mut self,
        py: Python<'py>,
        jointpos: PyReadonlyArray1<'py, f64>
    ) -> PyResult<Bound<'py, PyTuple>> {
        let x: Vec<f64> = jointpos.as_array().to_vec();
        let (p, q)  = self.inner.vars.robot.arms[0].get_ee_pos_and_quat_immutable(&x);
        let p_array = PyArray1::from_slice_bound(py, &[p.x, p.y, p.z]);
        let q_array = PyArray1::from_slice_bound(py, &[q.w, q.i, q.j, q.k]);
        return Ok(PyTuple::new_bound(py, &[p_array, q_array]));
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
