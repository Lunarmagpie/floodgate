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
        let micros = self.get_microseconds() as u64;
        Duration::from_secs(days * 86400 + seconds) + Duration::from_micros(micros)
    }
}

pub trait ToPydelta {
    fn as_pydelta<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDelta>;
}

impl ToPydelta for Duration {
    fn as_pydelta<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDelta> {
        PyDelta::new(
            py,
            0,
            self.as_secs() as i32,
            self.subsec_micros() as i32,
            false,
        )
    }
}
