#![allow(non_local_definitions)] // False positive from pyo3 macros

use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Mutex;

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
    // File operations
    m.add_function(wrap_pyfunction!(read_file_async, m)?)?;
    m.add_function(wrap_pyfunction!(write_file_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_file_bytes_async, m)?)?;
    m.add_function(wrap_pyfunction!(write_file_bytes_async, m)?)?;
    m.add_function(wrap_pyfunction!(append_file_async, m)?)?;
    m.add_function(wrap_pyfunction!(open_file, m)?)?;
    m.add_class::<AsyncFile>()?;
    
    // Directory operations
    m.add_function(wrap_pyfunction!(create_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(create_dir_all_async, m)?)?;
    m.add_function(wrap_pyfunction!(remove_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(remove_dir_all_async, m)?)?;
    m.add_function(wrap_pyfunction!(list_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(exists_async, m)?)?;
    m.add_function(wrap_pyfunction!(is_file_async, m)?)?;
    m.add_function(wrap_pyfunction!(is_dir_async, m)?)?;
    
    // Metadata operations
    m.add_function(wrap_pyfunction!(stat_async, m)?)?;
    m.add_function(wrap_pyfunction!(metadata_async, m)?)?;
    m.add_class::<FileMetadata>()?;
    
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

/// Async binary file read using Tokio (GIL-independent).
#[pyfunction]
fn read_file_bytes_async(py: Python<'_>, path: String) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let future = async move {
        let path_clone = path.clone();
        tokio::fs::read(&path).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to read file {}: {e}",
                path_clone
            ))
        })
    };
    future_into_py(py, future)
}

/// Async binary file write using Tokio (GIL-independent).
#[pyfunction]
fn write_file_bytes_async(py: Python<'_>, path: String, contents: &Bound<'_, PyBytes>) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let bytes = contents.as_bytes().to_vec();
    let future = async move {
        let path_clone = path.clone();
        tokio::fs::write(&path, bytes).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to write file {}: {e}",
                path_clone
            ))
        })
    };
    future_into_py(py, future)
}

/// Async file append using Tokio (GIL-independent).
#[pyfunction]
fn append_file_async(py: Python<'_>, path: String, contents: String) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let future = async move {
        let path_clone = path.clone();
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .await
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Failed to open file {} for appending: {e}",
                    path_clone
                ))
            })?;
        
        use tokio::io::AsyncWriteExt;
        file.write_all(contents.as_bytes()).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to append to file {}: {e}",
                path_clone
            ))
        })
    };
    future_into_py(py, future)
}

// Directory operations

/// Create a directory asynchronously.
#[pyfunction]
fn create_dir_async(py: Python<'_>, path: String) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let future = async move {
        let path_clone = path.clone();
        tokio::fs::create_dir(&path).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to create directory {}: {e}",
                path_clone
            ))
        })
    };
    future_into_py(py, future)
}

/// Create a directory and all parent directories asynchronously.
#[pyfunction]
fn create_dir_all_async(py: Python<'_>, path: String) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let future = async move {
        let path_clone = path.clone();
        tokio::fs::create_dir_all(&path).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to create directory {}: {e}",
                path_clone
            ))
        })
    };
    future_into_py(py, future)
}

/// Remove an empty directory asynchronously.
#[pyfunction]
fn remove_dir_async(py: Python<'_>, path: String) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let future = async move {
        let path_clone = path.clone();
        tokio::fs::remove_dir(&path).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to remove directory {}: {e}",
                path_clone
            ))
        })
    };
    future_into_py(py, future)
}

/// Remove a directory and all its contents asynchronously.
#[pyfunction]
fn remove_dir_all_async(py: Python<'_>, path: String) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let future = async move {
        let path_clone = path.clone();
        tokio::fs::remove_dir_all(&path).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to remove directory {}: {e}",
                path_clone
            ))
        })
    };
    future_into_py(py, future)
}

/// List directory contents asynchronously.
#[pyfunction]
fn list_dir_async(py: Python<'_>, path: String) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let future = async move {
        let path_clone = path.clone();
        let mut entries = tokio::fs::read_dir(&path).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to read directory {}: {e}",
                path_clone
            ))
        })?;
        
        let mut names = Vec::new();
        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to read directory entry in {}: {e}",
                path_clone
            ))
        })? {
            if let Some(name) = entry.file_name().to_str() {
                names.push(name.to_string());
            }
        }
        Ok(names)
    };
    future_into_py(py, future)
}

/// Check if a path exists asynchronously.
#[pyfunction]
fn exists_async(py: Python<'_>, path: String) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let future = async move {
        Ok(tokio::fs::metadata(&path).await.is_ok())
    };
    future_into_py(py, future)
}

/// Check if a path is a file asynchronously.
#[pyfunction]
fn is_file_async(py: Python<'_>, path: String) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let future = async move {
        let path_clone = path.clone();
        let metadata = tokio::fs::metadata(&path).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to get metadata for {}: {e}",
                path_clone
            ))
        })?;
        Ok(metadata.is_file())
    };
    future_into_py(py, future)
}

/// Check if a path is a directory asynchronously.
#[pyfunction]
fn is_dir_async(py: Python<'_>, path: String) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let future = async move {
        let path_clone = path.clone();
        let metadata = tokio::fs::metadata(&path).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to get metadata for {}: {e}",
                path_clone
            ))
        })?;
        Ok(metadata.is_dir())
    };
    future_into_py(py, future)
}

/// Parse file mode string to determine open options.
fn parse_mode(mode: &str) -> PyResult<(bool, bool, bool)> {
    // Returns (read, write, append)
    match mode {
        "r" => Ok((true, false, false)),
        "r+" => Ok((true, true, false)),
        "w" => Ok((false, true, false)),
        "w+" => Ok((true, true, false)),
        "a" => Ok((false, true, true)),
        "a+" => Ok((true, true, true)),
        "rb" => Ok((true, false, false)),
        "rb+" => Ok((true, true, false)),
        "wb" => Ok((false, true, false)),
        "wb+" => Ok((true, true, false)),
        "ab" => Ok((false, true, true)),
        "ab+" => Ok((true, true, true)),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Invalid mode: {mode}. Must be one of: r, r+, w, w+, a, a+, rb, rb+, wb, wb+, ab, ab+"),
        )),
    }
}

/// Async file handle for true async I/O operations.
#[pyclass]
struct AsyncFile {
    file: Arc<Mutex<File>>,
    path: String,
    mode: String,
}

#[pymethods]
impl AsyncFile {
    /// Open a file asynchronously.
    #[new]
    #[pyo3(signature = (path, mode = "r", buffering = -1, encoding = None, errors = None, newline = None, closefd = true, opener = None))]
    fn new(
        py: Python<'_>,
        path: String,
        mode: String,
        buffering: i32,
        encoding: Option<String>,
        errors: Option<String>,
        newline: Option<String>,
        closefd: bool,
        opener: Option<PyObject>,
    ) -> PyResult<Bound<'_, PyAny>> {
        // Validate parameters
        validate_path(&path)?;
        
        // Note: encoding, errors, newline, buffering, closefd, opener are accepted for API compatibility
        // but not fully implemented yet (will be added in later phases)
        if encoding.is_some() || errors.is_some() || newline.is_some() || !closefd || opener.is_some() {
            // For now, we'll accept these but not use them
            // TODO: Implement encoding/errors/newline handling in later phase
        }
        
        let (read, write, append) = parse_mode(&mode)?;
        let path_clone = path.clone();
        let mode_clone = mode.clone();
        
        let future = async move {
            let mut open_options = tokio::fs::OpenOptions::new();
            open_options.read(read);
            open_options.write(write || append);
            open_options.create(write || append);
            open_options.truncate(write && !append);
            open_options.append(append);
            
            let file = open_options.open(&path_clone).await.map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Failed to open file {}: {e}",
                    path_clone
                ))
            })?;
            
            Ok(AsyncFile {
                file: Arc::new(Mutex::new(file)),
                path: path_clone,
                mode: mode_clone,
            })
        };
        
        future_into_py(py, future)
    }
    
    /// Read from file.
    #[pyo3(signature = (size = -1))]
    fn read(&self, py: Python<'_>, size: i64) -> PyResult<Bound<'_, PyAny>> {
        let file = Arc::clone(&self.file);
        let path = self.path.clone();
        let mode = self.mode.clone();
        
        let future = async move {
            let mut file_guard = file.lock().await;
            
            let buffer = if size < 0 {
                // Read all
                let mut buffer = Vec::new();
                file_guard.read_to_end(&mut buffer).await.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                        "Failed to read file {}: {e}",
                        path
                    ))
                })?;
                buffer
            } else {
                let mut buffer = vec![0u8; size as usize];
                let n = file_guard.read(&mut buffer).await.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                        "Failed to read file {}: {e}",
                        path
                    ))
                })?;
                buffer.truncate(n);
                buffer
            };
            
            // Return bytes for binary modes, string for text modes
            if mode.contains('b') {
                Ok(buffer)
            } else {
                // Try to decode as UTF-8
                String::from_utf8(buffer).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyUnicodeDecodeError, _>(format!(
                        "Failed to decode file {} as UTF-8: {e}",
                        path
                    ))
                })
            }
        };
        
        future_into_py(py, future)
    }
    
    /// Write to file.
    fn write(&self, py: Python<'_>, data: &Bound<'_, PyAny>) -> PyResult<Bound<'_, PyAny>> {
        let file = Arc::clone(&self.file);
        let path = self.path.clone();
        
        // Convert Python bytes/string to Vec<u8>
        let bytes: Vec<u8> = if let Ok(py_bytes) = data.downcast::<PyBytes>() {
            py_bytes.as_bytes().to_vec()
        } else if let Ok(py_str) = data.downcast::<PyString>() {
            py_str.to_string().into_bytes()
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "write() argument must be bytes or str",
            ));
        };
        
        let future = async move {
            let mut file_guard = file.lock().await;
            file_guard.write_all(&bytes).await.map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Failed to write file {}: {e}",
                    path
                ))
            })?;
            Ok(bytes.len() as i64)
        };
        
        future_into_py(py, future)
    }
    
    /// Read a line from file.
    #[pyo3(signature = (size = -1))]
    fn readline(&self, py: Python<'_>, size: i64) -> PyResult<Bound<'_, PyAny>> {
        let file = Arc::clone(&self.file);
        let path = self.path.clone();
        let mode = self.mode.clone();
        
        let future = async move {
            let mut file_guard = file.lock().await;
            let mut buffer = Vec::new();
            let mut single_byte = [0u8; 1];
            
            loop {
                let n = file_guard.read(&mut single_byte).await.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                        "Failed to read file {}: {e}",
                        path
                    ))
                })?;
                
                if n == 0 {
                    break; // EOF
                }
                
                buffer.push(single_byte[0]);
                
                if single_byte[0] == b'\n' {
                    break; // End of line
                }
                
                if size > 0 && buffer.len() >= size as usize {
                    break; // Reached size limit
                }
            }
            
            // Return bytes for binary modes, string for text modes
            if mode.contains('b') {
                Ok(buffer)
            } else {
                String::from_utf8(buffer).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyUnicodeDecodeError, _>(format!(
                        "Failed to decode file {} as UTF-8: {e}",
                        path
                    ))
                })
            }
        };
        
        future_into_py(py, future)
    }
    
    /// Read all lines from file.
    #[pyo3(signature = (hint = -1))]
    fn readlines(&self, py: Python<'_>, hint: i64) -> PyResult<Bound<'_, PyAny>> {
        let file = Arc::clone(&self.file);
        let path = self.path.clone();
        let mode = self.mode.clone();
        
        let future = async move {
            let mut file_guard = file.lock().await;
            let mut lines = Vec::new();
            let mut current_line = Vec::new();
            let mut single_byte = [0u8; 1];
            
            loop {
                let n = file_guard.read(&mut single_byte).await.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                        "Failed to read file {}: {e}",
                        path
                    ))
                })?;
                
                if n == 0 {
                    if !current_line.is_empty() {
                        lines.push(current_line);
                    }
                    break; // EOF
                }
                
                current_line.push(single_byte[0]);
                
                if single_byte[0] == b'\n' {
                    lines.push(current_line);
                    current_line = Vec::new();
                    
                    if hint > 0 && lines.len() >= hint as usize {
                        break;
                    }
                }
            }
            
            // Return list of bytes for binary modes, list of strings for text modes
            if mode.contains('b') {
                Ok(lines)
            } else {
                let string_lines: Result<Vec<String>, _> = lines
                    .into_iter()
                    .map(|line| String::from_utf8(line))
                    .collect();
                string_lines.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyUnicodeDecodeError, _>(format!(
                        "Failed to decode file {} as UTF-8: {e}",
                        path
                    ))
                })
            }
        };
        
        future_into_py(py, future)
    }
    
    /// Seek to a position in the file.
    #[pyo3(signature = (offset, whence = 0))]
    fn seek(&self, py: Python<'_>, offset: i64, whence: i32) -> PyResult<Bound<'_, PyAny>> {
        let file = Arc::clone(&self.file);
        let path = self.path.clone();
        
        let future = async move {
            let mut file_guard = file.lock().await;
            
            let pos = match whence {
                0 => std::io::SeekFrom::Start(offset as u64),
                1 => std::io::SeekFrom::Current(offset),
                2 => std::io::SeekFrom::End(offset),
                _ => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        format!("Invalid whence value: {whence}. Must be 0 (SEEK_SET), 1 (SEEK_CUR), or 2 (SEEK_END)"),
                    ));
                }
            };
            
            let new_pos = file_guard.seek(pos).await.map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Failed to seek in file {}: {e}",
                    path
                ))
            })?;
            
            Ok(new_pos as i64)
        };
        
        future_into_py(py, future)
    }
    
    /// Get current position in file.
    fn tell(&self, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        let file = Arc::clone(&self.file);
        let path = self.path.clone();
        
        let future = async move {
            let mut file_guard = file.lock().await;
            let pos = file_guard.stream_position().await.map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Failed to get position in file {}: {e}",
                    path
                ))
            })?;
            Ok(pos as i64)
        };
        
        future_into_py(py, future)
    }
    
    /// Close the file.
    fn close(&self, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        // File is automatically closed when dropped, but we provide this for API compatibility
        let future = async move {
            // The file will be closed when the Arc is dropped
            Ok(())
        };
        future_into_py(py, future)
    }
    
    /// Async context manager entry.
    fn __aenter__(&self, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        // Return self for context manager - create a future that yields self
        // We clone the necessary data to reconstruct self in the future
        let file = Arc::clone(&self.file);
        let path = self.path.clone();
        let mode = self.mode.clone();
        
        let future = async move {
            // Reconstruct AsyncFile with cloned data
            Ok(AsyncFile {
                file,
                path,
                mode,
            })
        };
        future_into_py(py, future)
    }
    
    /// Async context manager exit.
    fn __aexit__(
        &self,
        py: Python<'_>,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'_, PyAny>> {
        // Close the file on exit, return False to not suppress exceptions
        let future = async move {
            Ok(false) // Return False to not suppress exceptions
        };
        future_into_py(py, future)
    }
}

/// File metadata structure (aiofiles.stat_result compatible).
#[pyclass]
#[derive(Clone)]
struct FileMetadata {
    size: u64,
    is_file: bool,
    is_dir: bool,
    modified: f64,  // Unix timestamp
    accessed: f64,  // Unix timestamp
    created: f64,   // Unix timestamp (creation time on Windows, birth time on Unix)
}

#[pymethods]
impl FileMetadata {
    #[new]
    fn new(
        size: u64,
        is_file: bool,
        is_dir: bool,
        modified: f64,
        accessed: f64,
        created: f64,
    ) -> Self {
        FileMetadata {
            size,
            is_file,
            is_dir,
            modified,
            accessed,
            created,
        }
    }
    
    #[getter]
    fn size(&self) -> u64 {
        self.size
    }
    
    #[getter]
    fn is_file(&self) -> bool {
        self.is_file
    }
    
    #[getter]
    fn is_dir(&self) -> bool {
        self.is_dir
    }
    
    #[getter]
    fn modified(&self) -> f64 {
        self.modified
    }
    
    #[getter]
    fn accessed(&self) -> f64 {
        self.accessed
    }
    
    #[getter]
    fn created(&self) -> f64 {
        self.created
    }
}

/// Convert SystemTime to Unix timestamp.
fn system_time_to_timestamp(time: SystemTime) -> f64 {
    time.duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

/// Get file statistics asynchronously.
#[pyfunction]
fn stat_async(py: Python<'_>, path: String) -> PyResult<Bound<'_, PyAny>> {
    validate_path(&path)?;
    let future = async move {
        let path_clone = path.clone();
        let metadata = tokio::fs::metadata(&path).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to get metadata for {}: {e}",
                path_clone
            ))
        })?;
        
        let size = metadata.len();
        let is_file = metadata.is_file();
        let is_dir = metadata.is_dir();
        
        let modified = metadata
            .modified()
            .map(system_time_to_timestamp)
            .unwrap_or(0.0);
        let accessed = metadata
            .accessed()
            .map(system_time_to_timestamp)
            .unwrap_or(0.0);
        
        // Creation time (available on Windows, birth time on Unix requires platform-specific code)
        let created = metadata
            .created()
            .map(system_time_to_timestamp)
            .unwrap_or(modified); // Fallback to modified time if creation time not available
        
        Ok(FileMetadata {
            size,
            is_file,
            is_dir,
            modified,
            accessed,
            created,
        })
    };
    future_into_py(py, future)
}

/// Get file metadata asynchronously (alias for stat).
#[pyfunction]
fn metadata_async(py: Python<'_>, path: String) -> PyResult<Bound<'_, PyAny>> {
    stat_async(py, path)
}

/// Open a file asynchronously (aiofiles.open() compatible).
#[pyfunction]
#[pyo3(signature = (path, mode = "r", buffering = -1, encoding = None, errors = None, newline = None, closefd = true, opener = None))]
fn open_file(
    py: Python<'_>,
    path: String,
    mode: String,
    buffering: i32,
    encoding: Option<String>,
    errors: Option<String>,
    newline: Option<String>,
    closefd: bool,
    opener: Option<PyObject>,
) -> PyResult<Bound<'_, PyAny>> {
    AsyncFile::new(py, path, mode, buffering, encoding, errors, newline, closefd, opener)
}
