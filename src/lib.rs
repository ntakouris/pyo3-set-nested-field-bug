use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct Beta {
    #[pyo3(get, set)]
    pub field: i32,
}

#[pymethods]
impl Beta {
    #[new]
    pub fn new(field: i32) -> Self {
        Beta { field }
    }
}

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct Alpha {
    #[pyo3(get, set)]
    pub beta: Beta,
}

#[pymethods]
impl Alpha {
    #[new]
    pub fn new(beta: Beta) -> Self {
        Alpha { beta }
    }
}

#[pymodule]
fn testlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Beta>()?;
    m.add_class::<Alpha>()?;

    Ok(())
}
