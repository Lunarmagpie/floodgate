use pyo3::prelude::*;

mod dynamic_mapping;
mod fixed_mapping;
mod jumping_window;
mod traits;

#[pymodule]
#[pyo3(name = "floodgate")]
fn pyfloodgate(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<fixed_mapping::PyFixedMapping>()?;
    m.add_class::<dynamic_mapping::PyDynamicMapping>()?;
    m.add_class::<jumping_window::PyJumpingWindow>()?;
    Ok(())
}
