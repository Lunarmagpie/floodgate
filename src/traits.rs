use pyo3::types::PyDelta;
use pyo3::types::PyDeltaAccess;
use pyo3::{PyResult, Python};
use std::time::Duration;

pub trait ToDuration {
    fn as_duration(&self) -> Duration;
}

impl ToDuration for PyDelta {
    fn as_duration(&self) -> Duration {
        let days = self.get_days() as u64;
        let seconds = self.get_seconds() as u64;
        let mircros = self.get_microseconds() as u64;
        Duration::from_micros((days * 86400 + seconds) * 1000000 + mircros)
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
