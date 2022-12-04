use crate::traits::{ToDuration, ToPydelta};

use pyo3::prelude::*;
use pyo3::types::PyDelta;
use std::sync::Arc;

#[pyclass(name = "FixedMapping")]
pub struct PyFixedMapping {
    mapping: Arc<floodgate::FixedMapping<isize>>,
}

#[pymethods]
impl PyFixedMapping {
    #[new]
    fn new(capacity: u64, period: &PyDelta) -> PyResult<Self> {
        let mapping = floodgate::FixedMapping::new(capacity, period.as_duration());
        Ok(Self {
            mapping: Arc::new(mapping),
        })
    }

    fn tokens(&self, key: &PyAny) -> PyResult<u64> {
        let hash = &key.hash()?;
        Ok(self.mapping.tokens(hash))
    }

    pub fn next_reset<'py>(&self, py: Python<'py>, key: &PyAny) -> PyResult<&'py PyDelta> {
        let hash = &key.hash()?;
        self.mapping.next_reset(hash).as_pydelta(py)
    }

    pub fn retry_after<'py>(&self, py: Python<'py>, key: &PyAny) -> PyResult<Option<&'py PyDelta>> {
        let hash = &key.hash()?;
        match self.mapping.retry_after(hash) {
            Some(retry) => Ok(Some(retry.as_pydelta(py)?)),
            None => Ok(None),
        }
    }

    pub fn can_trigger(&self, key: &PyAny) -> PyResult<bool> {
        let hash = &key.hash()?;
        Ok(self.mapping.can_trigger(hash))
    }

    pub fn trigger<'py>(&self, py: Python<'py>, key: &PyAny) -> PyResult<Option<&'py PyDelta>> {
        let hash = &key.hash()?;
        match self.mapping.trigger(hash) {
            Some(retry) => Ok(Some(retry.as_pydelta(py)?)),
            None => Ok(None),
        }
    }

    pub fn reset(&self, key: &PyAny) -> PyResult<()> {
        let hash = &key.hash()?;
        self.mapping.reset(hash);
        Ok(())
    }

    #[args(cycle_period = "None")]
    pub fn start(&self, cycle_period: Option<&PyDelta>) {
        let duration = cycle_period.map(PyDelta::as_duration);
        let mapping = self.mapping.clone();
        floodgate::FixedMapping::<isize>::start(mapping, duration);
    }
}
