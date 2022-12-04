use std::time::Duration;

use floodgate::{self, FixedMapping};
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass(name = "FixedMapping")]
struct PyFixedMapping {
    mapping: Arc<floodgate::FixedMapping<isize>>,
}

#[pymethods]
impl PyFixedMapping {
    #[new]
    fn new(capacity: u64, period: f64) -> PyResult<Self> {
        let mapping = floodgate::FixedMapping::new(capacity, Duration::from_secs_f64(period));
        Ok(Self {
            mapping: Arc::new(mapping),
        })
    }

    fn tokens(&self, key: &PyAny) -> PyResult<u64> {
        let hash = &key.hash()?;
        Ok(self.mapping.tokens(hash))
    }

    pub fn next_reset(&self, key: &PyAny) -> PyResult<f64> {
        let hash = &key.hash()?;
        Ok(self.mapping.next_reset(hash).as_secs_f64())
    }

    pub fn retry_after(&self, key: &PyAny) -> PyResult<Option<f64>> {
        let hash = &key.hash()?;
        Ok(self.mapping.retry_after(hash).map(|d| d.as_secs_f64()))
    }

    pub fn can_trigger(&self, key: &PyAny) -> PyResult<bool> {
        let hash = &key.hash()?;
        Ok(self.mapping.can_trigger(hash))
    }

    pub fn trigger(&self, key: &PyAny) -> PyResult<Option<f64>> {
        let hash = &key.hash()?;
        Ok(self.mapping.trigger(hash).map(|d| d.as_secs_f64()))
    }

    pub fn reset(&self, key: &PyAny) -> PyResult<()> {
        let hash = &key.hash()?;
        self.mapping.reset(hash);
        Ok(())
    }

    pub fn start(&self, cycle_period: Option<f64>) {
        let duration = cycle_period.map(Duration::from_secs_f64);
        let mapping = self.mapping.clone();
        FixedMapping::<isize>::start(mapping, duration);
    }
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "floodgate")]
fn pyfloodgate(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyFixedMapping>()?;
    Ok(())
}

mod tests {
    use std::time::Duration;

    #[test]
    fn benchmark() {
        let mapping = floodgate::FixedMapping::<u64>::new(1, Duration::from_secs(2));
        for _ in 0..100_000_00 {
            mapping.trigger(&1234);
        }
    }
}

