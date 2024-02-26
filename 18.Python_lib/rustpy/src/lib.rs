use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn dict_make(py:Python, keys:Vec<&str>, val:Vec<isize>) -> PyResult<PyObject>{
    let result = PyDict::new(py);
    for i in 0..keys.len(){
        result.set_item(keys[i], val[i])?;
    }
    Ok(result.into())
}
#[pyfunction]
fn hash2dict(keys:Vec<&str>, val:Vec<isize>) -> PyResult<HashMap<&str,isize>>{
    let mut result = HashMap::new();
    for i in 0..keys.len(){
        result.insert(keys[i], val[i]);
    }
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dict_make, m)?)?;
    m.add_function(wrap_pyfunction!(hash2dict, m)?)?;
    Ok(())
}
