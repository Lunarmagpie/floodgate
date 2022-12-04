use pyo3::{Python, PyResult};
use pyo3::types::PyDelta;
use pyo3::types::PyDeltaAccess;
use std::time::Duration;

pub trait ToDuration {
    fn as_duration(&self) -> Duration;
}

impl ToDuration for PyDelta {
    fn as_duration(&self) -> Duration {
        Duration::from_micros(self.get_microseconds() as u64)
    }
}

pub trait ToPydelta {
    fn as_pydelta<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDelta>;
}

impl ToPydelta for Duration {
    fn as_pydelta<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDelta> {
        PyDelta::new(py, 0, 0, self.as_micros() as i32, false)
    }
}
