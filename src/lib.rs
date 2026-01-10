use pyo3::prelude::*;
use pyo3_asyncio::tokio::into_future;

/// Python bindings for rapfiles - True async filesystem I/O.
#[pymodule]
fn _rapfiles(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_file_async, m)?)?;
    m.add_function(wrap_pyfunction!(write_file_async, m)?)?;
    Ok(())
}

/// Async file read using Tokio (GIL-independent).
#[pyfunction]
fn read_file_async(path: String) -> PyResult<&PyAny> {
    let future = async move {
        tokio::fs::read_to_string(path)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to read file: {}", e)))
    };
    into_future(future)
}

/// Async file write using Tokio (GIL-independent).
#[pyfunction]
fn write_file_async(path: String, contents: String) -> PyResult<&PyAny> {
    let future = async move {
        tokio::fs::write(path, contents)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to write file: {}", e)))
    };
    into_future(future)
}
