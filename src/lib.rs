#![allow(non_local_definitions)] // False positive from pyo3 macros

use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py;

/// Validate a file path for security and correctness.
fn validate_path(path: &str) -> PyResult<()> {
    if path.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Path cannot be empty",
        ));
    }
    if path.contains('\0') {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Path cannot contain null bytes",
        ));
    }
    Ok(())
}

/// Python bindings for rapfiles - True async filesystem I/O.
#[pymodule]
fn _rapfiles(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_file_async, m)?)?;
    m.add_function(wrap_pyfunction!(write_file_async, m)?)?;
    Ok(())
}

/// Async file read using Tokio (GIL-independent).
#[pyfunction]
fn read_file_async(py: Python<'_>, path: String) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let future = async move {
        let path_clone = path.clone();
        tokio::fs::read_to_string(&path).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to read file {}: {e}",
                path_clone
            ))
        })
    };
    future_into_py(py, future)
}

/// Async file write using Tokio (GIL-independent).
#[pyfunction]
fn write_file_async(py: Python<'_>, path: String, contents: String) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let future = async move {
        let path_clone = path.clone();
        tokio::fs::write(&path, contents).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to write file {}: {e}",
                path_clone
            ))
        })
    };
    future_into_py(py, future)
}
