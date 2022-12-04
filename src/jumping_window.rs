use crate::traits::{ToDuration, ToPydelta};

use pyo3::prelude::*;
use pyo3::types::PyDelta;

#[pyclass(name = "JumpingWindow")]
pub struct PyJumpingWindow {
    window: floodgate::JumpingWindow,
}

#[pymethods]
impl PyJumpingWindow {
    #[new]
    fn new(capacity: u64, period: &PyDelta) -> Self {
        Self {
            window: floodgate::JumpingWindow::new(capacity, period.as_duration()),
        }
    }

    fn token(&mut self) -> u64 {
        self.window.tokens(None)
    }

    fn next_reset<'py>(&mut self, py: Python<'py>) -> PyResult<&'py PyDelta> {
        self.window.next_reset(None).as_pydelta(py)
    }

    fn retry_after<'py>(&mut self, py: Python<'py>) -> PyResult<Option<&'py PyDelta>> {
        match self.window.retry_after(None) {
            Some(t) => Ok(Some(t.as_pydelta(py)?)),
            None => Ok(None),
        }
    }

    fn can_trigger(&mut self) -> bool {
        self.window.can_trigger(None)
    }

    fn trigger<'py>(&mut self, py: Python<'py>) -> PyResult<Option<&'py PyDelta>> {
        match self.window.trigger(None) {
            Some(t) => Ok(Some(t.as_pydelta(py)?)),
            None => Ok(None),
        }
    }

    fn reset(&mut self) {
        self.window.reset(None)
    }
}
