# rapfiles

**True async filesystem I/O — no fake async, no GIL stalls.**

[![PyPI version](https://img.shields.io/pypi/v/rapfiles.svg)](https://pypi.org/project/rapfiles/)
[![Downloads](https://pepy.tech/badge/rapfiles)](https://pepy.tech/project/rapfiles)
[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

`rapfiles` provides true async filesystem I/O operations for Python, backed by Rust and Tokio. Unlike libraries that wrap blocking I/O in `async` syntax, `rapfiles` guarantees that all I/O work executes **outside the Python GIL**, ensuring event loops never stall under load.

**Roadmap Goal**: Achieve drop-in replacement compatibility with `aiofiles`, enabling seamless migration with true async performance. See [ROADMAP.md](https://github.com/eddiethedean/rapfiles/blob/main/ROADMAP.md) for details.

## Why `rap*`?

Packages prefixed with **`rap`** stand for **Real Async Python**. Unlike many libraries that merely wrap blocking I/O in `async` syntax, `rap*` packages guarantee that all I/O work is executed **outside the Python GIL** using native runtimes (primarily Rust). This means event loops are never stalled by hidden thread pools, blocking syscalls, or cooperative yielding tricks. If a `rap*` API is `async`, it is *structurally non-blocking by design*, not by convention. The `rap` prefix is a contract: measurable concurrency, real parallelism, and verifiable async behavior under load.

See the [rap-manifesto](https://github.com/eddiethedean/rap-manifesto) for philosophy and guarantees.

## Features

- ✅ **True async** file reads and writes
- ✅ **Native Rust-backed** execution (Tokio)
- ✅ **Zero Python thread pools**
- ✅ **Event-loop-safe** concurrency under load
- ✅ **GIL-independent** I/O operations
- ✅ **Verified** by Fake Async Detector

## Requirements

- Python 3.8+
- Rust 1.70+ (for building from source)

## Installation

```bash
pip install rapfiles
```

### Building from Source

```bash
git clone https://github.com/eddiethedean/rapfiles.git
cd rapfiles
pip install maturin
maturin develop
```

---

## Usage

```python
import asyncio
from rapfiles import read_file, write_file

async def main():
    # Write file asynchronously (true async, GIL-independent)
    await write_file("example.txt", "Hello from rapfiles!")
    
    # Read file asynchronously (true async, GIL-independent)
    content = await read_file("example.txt")
    print(content)  # Output: Hello from rapfiles!
    
    # Write another file
    await write_file("output.txt", content)

asyncio.run(main())
```

### Concurrent File Operations

```python
import asyncio
from rapfiles import read_file, write_file

async def main():
    # Process multiple files concurrently
    tasks = [
        write_file("file1.txt", "Content 1"),
        write_file("file2.txt", "Content 2"),
        write_file("file3.txt", "Content 3"),
    ]
    await asyncio.gather(*tasks)
    
    # Read all files concurrently
    contents = await asyncio.gather(
        read_file("file1.txt"),
        read_file("file2.txt"),
        read_file("file3.txt"),
    )
    print(contents)  # ['Content 1', 'Content 2', 'Content 3']

asyncio.run(main())
```

## API Reference

### `read_file(path: str) -> str`

Read a file asynchronously and return its contents as a string.

**Parameters:**
- `path` (str): Path to the file to read

**Returns:**
- `str`: File contents

**Raises:**
- `IOError`: If the file cannot be read

### `write_file(path: str, contents: str) -> None`

Write content to a file asynchronously.

**Parameters:**
- `path` (str): Path to the file to write
- `contents` (str): Content to write to the file

**Raises:**
- `IOError`: If the file cannot be written

## Benchmarks

This package passes the [Fake Async Detector](https://github.com/eddiethedean/rap-bench). Benchmarks are available in the [rap-bench](https://github.com/eddiethedean/rap-bench) repository.

Run the detector yourself:

```bash
pip install rap-bench
rap-bench detect rapfiles
```

## Roadmap

See [ROADMAP.md](https://github.com/eddiethedean/rapfiles/blob/main/ROADMAP.md) for detailed development plans. Key goals include:
- Drop-in replacement for `aiofiles` (Phase 1)
- Comprehensive filesystem operations (directories, metadata, permissions)
- Advanced I/O patterns and zero-copy optimizations
- Filesystem traversal and watching capabilities

## Related Projects

- [rap-manifesto](https://github.com/eddiethedean/rap-manifesto) - Philosophy and guarantees
- [rap-bench](https://github.com/eddiethedean/rap-bench) - Fake Async Detector CLI
- [rapsqlite](https://github.com/eddiethedean/rapsqlite) - True async SQLite
- [rapcsv](https://github.com/eddiethedean/rapcsv) - Streaming async CSV

## Limitations (v0.0.2)

**Current limitations:**
- Only basic `read_file()` and `write_file()` operations
- No directory operations (listing, creation, deletion)
- No file metadata operations (stat, permissions, timestamps)
- No filesystem traversal or navigation
- Not yet a drop-in replacement for `aiofiles` (goal for Phase 1)
- Not designed for synchronous use cases

**Recent improvements (v0.0.2):**
- ✅ Security fixes: Upgraded dependencies (pyo3 0.27, pyo3-async-runtimes 0.27)
- ✅ Input validation: Added path validation (non-empty, no null bytes)
- ✅ Improved error handling: Enhanced error messages with file path context
- ✅ Type stubs: Added `.pyi` type stubs for better IDE support and type checking

**Roadmap**: See [ROADMAP.md](https://github.com/eddiethedean/rapfiles/blob/main/ROADMAP.md) for planned improvements. Our goal is to achieve drop-in replacement compatibility with `aiofiles` while providing true async performance with GIL-independent I/O.

## Contributing

Contributions are welcome! Please see our [contributing guidelines](https://github.com/eddiethedean/rapfiles/blob/main/CONTRIBUTING.md) (coming soon).

## License

MIT

## Changelog

See [CHANGELOG.md](https://github.com/eddiethedean/rapfiles/blob/main/CHANGELOG.md) (coming soon) for version history.
