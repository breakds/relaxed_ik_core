pub mod utils_rust;
pub mod spacetime;
pub mod groove;
pub mod relaxed_ik;
pub mod relaxed_ik_wrapper;
pub mod relaxed_ik_web;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn get_path_to_src() -> PyResult<String> {
    Ok(utils_rust::file_utils::get_path_to_src())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn relaxed_ik_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_path_to_src, m)?)?;
    Ok(())
}
