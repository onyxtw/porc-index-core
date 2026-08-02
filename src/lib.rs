pub mod compute;
pub mod evaluator;
pub mod resilience;
pub mod thermal;

pub use evaluator::PorcEvaluator;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyfunction]
fn compute_porc_index_py(json_str: &str) -> PyResult<(f64, String, f64, f64, f64, f64)> {
let evaluator = PorcEvaluator::from_json(json_str)
.map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

let (c, t, r, phi) = evaluator.calculate_sub_indices();
let (score, tier) = evaluator.compute_index();

Ok((score, tier, c, t, r, phi))
}

#[cfg(feature = "python")]
#[pymodule]
fn _native(_py: Python, m: &PyModule) -> PyResult<()> {
m.add_function(wrap_pyfunction!(compute_porc_index_py, m)?)?;
Ok(())
}
