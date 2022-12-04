use crate::traits::{ToDuration, ToPydelta};

use pyo3::prelude::*;
use pyo3::types::PyDelta;
use std::sync::Arc;

#[pyclass(name = "DynamicMapping")]
pub struct PyDynamicMapping {
    mapping: Arc<floodgate::DynamicMapping<isize>>,
}

#[pymethods]
impl PyDynamicMapping {
    #[new]
    fn new(max_period: &PyDelta) -> Self {
        let mapping = Arc::new(floodgate::DynamicMapping::new(max_period.as_duration()));
        floodgate::DynamicMapping::start(mapping.clone());
        Self { mapping }
    }

    fn tokens(&self, key: &PyAny, capacity: u64, duration: &PyDelta) -> PyResult<u64> {
        let hash = &key.hash()?;
        Ok(self.mapping.tokens(hash, capacity, duration.as_duration()))
    }

    pub fn next_reset<'py>(
        &self,
        py: Python<'py>,
        key: &PyAny,
        capacity: u64,
        duration: &PyDelta,
    ) -> PyResult<&'py PyDelta> {
        let hash = &key.hash()?;
        self.mapping
            .next_reset(hash, capacity, duration.as_duration())
            .as_pydelta(py)
    }

    pub fn retry_after<'py>(
        &self,
        py: Python<'py>,
        key: &PyAny,
        capacity: u64,
        duration: &PyDelta,
    ) -> PyResult<Option<&'py PyDelta>> {
        let hash = &key.hash()?;
        match self
            .mapping
            .retry_after(hash, capacity, duration.as_duration())
        {
            Some(retry) => Ok(Some(retry.as_pydelta(py)?)),
            None => Ok(None),
        }
    }

    pub fn can_trigger(&self, key: &PyAny, capacity: u64, duration: &PyDelta) -> PyResult<bool> {
        let hash = &key.hash()?;
        Ok(self
            .mapping
            .can_trigger(hash, capacity, duration.as_duration()))
    }

    pub fn trigger<'py>(
        &self,
        py: Python<'py>,
        key: &PyAny,
        capacity: u64,
        duration: &PyDelta,
    ) -> PyResult<Option<&'py PyDelta>> {
        let hash = &key.hash()?;
        match self.mapping.trigger(hash, capacity, duration.as_duration()) {
            Some(retry) => Ok(Some(retry.as_pydelta(py)?)),
            None => Ok(None),
        }
    }

    pub fn reset(&self, key: &PyAny, capacity: u64, duration: &PyDelta) -> PyResult<()> {
        let hash = &key.hash()?;
        self.mapping.reset(hash, capacity, duration.as_duration());
        Ok(())
    }
}
