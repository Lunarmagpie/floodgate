use pyo3::prelude::*;

mod dynamic_mapping;
mod fixed_mapping;
mod traits;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "floodgate")]
fn pyfloodgate(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<fixed_mapping::PyFixedMapping>()?;
    m.add_class::<dynamic_mapping::PyDynamicMapping>()?;
    Ok(())
}
