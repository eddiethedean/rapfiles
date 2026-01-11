# rapfiles Roadmap

This roadmap outlines the development plan for `rapfiles`, aligned with the [RAP Project Strategic Plan](../rap-project-plan.md). `rapfiles` provides true async filesystem I/O operations for Python, backed by Rust and Tokio.

## Current Status

**Current Version (v0.0.2)** - Current limitations:

- Only basic `read_file()` and `write_file()` operations
- No directory operations (listing, creation, deletion)
- No file metadata operations (stat, permissions, timestamps)
- No file system navigation or traversal
- No symbolic link handling
- No concurrent batch operations API
- Not yet a drop-in replacement for `aiofiles`

**Recent improvements (v0.0.2):**
- ✅ Security fixes: Upgraded dependencies (pyo3 0.27, pyo3-async-runtimes 0.27)
- ✅ Input validation: Added path validation (non-empty, no null bytes)
- ✅ Improved error handling: Enhanced error messages with file path context
- ✅ Type stubs: Added `.pyi` type stubs for better IDE support and type checking

**Goal**: Achieve drop-in replacement compatibility with `aiofiles` to enable seamless migration with true async performance.

## Phase 1 — Credibility

Focus: Expand core filesystem operations beyond basic read/write to establish a solid foundation.

### Core File Operations

- **File handle management**
  - Context manager support (`async with` for file handles)
  - Explicit file handle objects with lifecycle management
  - Proper resource cleanup and error handling
  - File handle reuse for multiple operations

- **Extended read/write operations**
  - `read_file_bytes()` - binary file reading
  - `write_file_bytes()` - binary file writing
  - `append_file()` - efficient append operations
  - `read_file_chunked()` - streaming read for large files
  - `write_file_chunked()` - streaming write for large files

- **File position and seeking**
  - Seek operations for random access
  - Tell/get current position
  - Efficient position management

### Directory Operations

- **Basic directory operations**
  - `create_dir()` / `create_dir_all()` - create directories
  - `remove_dir()` / `remove_dir_all()` - remove directories
  - `list_dir()` - list directory contents
  - `exists()` - check file/directory existence
  - `is_file()` / `is_dir()` - type checking

- **Directory traversal basics**
  - Recursive directory listing
  - Filter-based directory walking
  - Basic path manipulation utilities

### File Metadata

- **Metadata operations**
  - `stat()` - file statistics (size, permissions, timestamps)
  - `set_permissions()` - change file permissions
  - `metadata()` - comprehensive file metadata
  - Cross-platform metadata handling

- **Path operations**
  - Path joining and normalization
  - Absolute/relative path conversion
  - Path existence and validation

### Error Handling & Stability

- **Enhanced error handling**
  - Filesystem-specific error types
  - Better error messages with context
  - Platform-specific error mapping
  - Error recovery strategies

- **API stability**
  - Consistent API patterns across operations
  - Resource management guarantees
  - Thread-safety documentation
  - Performance characteristics documented

### API Compatibility for Drop-In Replacement

- **aiofiles API compatibility**
  - Match `aiofiles.open()`, `aiofiles.ospath`, and `aiofiles.oswrap` APIs
  - Compatible file handle objects (matching `aiofiles.AIOFiles` interface)
  - Matching function signatures (`read()`, `write()`, `seek()`, `tell()`, etc.)
  - Compatible exception types and error behavior
  - Compatible context manager behavior
  - Drop-in replacement validation: `import rapfiles as aiofiles` compatibility tests

- **Migration support**
  - Compatibility shim/adapter layer if needed for exact API matching
  - Migration guide documenting any differences
  - Backward compatibility considerations

### Testing & Validation

- Comprehensive test suite covering edge cases
- Fake Async Detector validation passes under load
- **Pass 100% of aiofiles test suite** as drop-in replacement validation
- Drop-in replacement compatibility tests (can swap `aiofiles` → `rapfiles` without code changes)
- Benchmark comparison with existing async file I/O libraries
- Documentation improvements including migration guide

## Phase 2 — Expansion

Focus: Advanced filesystem features, performance optimizations, and broader compatibility.

### Advanced File Operations

- **File manipulation**
  - `copy_file()` - efficient file copying
  - `move_file()` / `rename()` - file moving and renaming
  - `remove_file()` - file deletion
  - `hard_link()` / `symlink()` - link operations
  - `canonicalize()` - resolve symbolic links

- **Atomic operations**
  - Atomic file writes (write to temp, then rename)
  - Atomic file moves
  - Transaction-like file operations

- **File locking**
  - Advisory file locks (shared/exclusive)
  - Cross-platform locking support
  - Lock timeout and error handling

### Directory & Filesystem Features

- **Advanced directory operations**
  - Recursive directory copying
  - Directory tree operations
  - Efficient batch directory operations
  - Directory watching (optional, platform-specific)

- **Filesystem traversal**
  - `walk_dir()` - recursive directory walking
  - Configurable traversal options (depth, filters, etc.)
  - Streaming directory traversal for large trees
  - Parallel directory traversal

### Concurrent Operations

- **Batch operations API**
  - `read_files()` - concurrent multi-file reading
  - `write_files()` - concurrent multi-file writing
  - `copy_files()` - concurrent file copying
  - Batch operation error handling and progress tracking

- **Streaming operations**
  - Large file streaming patterns
  - Memory-efficient batch processing
  - Backpressure handling
  - Progress callbacks for long operations

### Performance & Optimization

- **Performance enhancements**
  - Zero-copy operations where possible
  - Efficient buffer management
  - Memory-mapped file support for large files
  - SIMD operations for bulk data operations (where applicable)

- **Benchmarking**
  - Comparison with `aiofiles`, `asyncio`, synchronous I/O
  - Throughput and latency metrics
  - Memory usage profiles
  - Concurrent operation benchmarks

### Compatibility & Integration

- **Additional API compatibility**
  - Maintain and refine aiofiles drop-in replacement (achieved in Phase 1)
  - Pathlib integration examples
  - Migration guides from other libraries (asyncio file I/O, etc.)
  - Enhanced compatibility features beyond core aiofiles API

- **Platform support**
  - Windows-specific optimizations
  - Unix/Linux-specific features
  - macOS compatibility
  - Cross-platform testing

## Phase 3 — Ecosystem

Focus: Advanced I/O patterns, ecosystem integration, and zero-copy optimizations.

### Advanced I/O Patterns

- **Streaming pipelines**
  - File stream transformations
  - Multi-stage file processing
  - Efficient data pipeline patterns
  - Backpressure-aware streaming

- **Memory-mapped files**
  - Memory-mapped file support for very large files
  - Shared memory patterns
  - Efficient random access to large files

- **Advanced patterns**
  - File deduplication utilities
  - Checksum calculation (MD5, SHA, etc.)
  - File comparison utilities
  - Archive operations (tar, zip reading/writing)

### Integration & Ecosystem

- **rap-core integration**
  - Shared primitives with other rap packages
  - Common I/O patterns and utilities
  - Unified error handling
  - Performance monitoring hooks

- **Framework compatibility**
  - Integration with web frameworks (FastAPI, aiohttp)
  - Integration with data processing frameworks
  - ETL pipeline patterns
  - Background task patterns

### Observability & Tooling

- **Monitoring & metrics**
  - Performance metrics export
  - Operation timing and profiling
  - Resource usage tracking
  - Debugging and diagnostic tools

- **Developer experience**
  - Comprehensive logging support
  - Debug modes and verbose output
  - Error diagnostics and suggestions
  - Performance profiling utilities

### Advanced Features

- **Filesystem watching**
  - Directory change notifications
  - File modification watching
  - Cross-platform file watching (optional dependency)
  - Event-driven file processing patterns

- **Distributed operations**
  - Remote file operations (if applicable)
  - Network filesystem support
  - Cloud storage integration patterns
  - Distributed file processing

### Documentation & Community

- **Comprehensive documentation**
  - Advanced usage patterns and examples
  - Performance tuning guides
  - Migration documentation from other libraries
  - Contributing guidelines and code examples

- **Ecosystem presence**
  - PyPI package optimization
  - CI/CD pipeline improvements
  - Community examples and tutorials
  - Blog posts and case studies
  - Conference talks and presentations

## Cross-Package Dependencies

- **Phase 1**: Independent development, minimal dependencies
- **Phase 2**: Potential integration with `rapcsv` for file-based CSV operations, `rapsqlite` for database file operations
- **Phase 3**: Integration with `rap-core` for shared primitives, serve as foundation for other rap packages

## Success Criteria

- **Phase 1**: Comprehensive file and directory operations, stable API, **drop-in replacement for aiofiles**, passes 100% of aiofiles test suite, passes Fake Async Detector under all load conditions
- **Phase 2**: Feature-complete for common filesystem use cases, competitive performance benchmarks, excellent documentation, seamless migration from aiofiles
- **Phase 3**: Industry-leading performance, ecosystem integration, adoption as foundation library for async Python filesystem operations and preferred aiofiles alternative

## Versioning Strategy

Following semantic versioning:
- `v0.x`: Breaking changes allowed, MVP and Phase 1 development
- `v1.0`: Stable API, Phase 1 complete, production-ready
- `v1.x+`: Phase 2 and 3 features, backwards-compatible additions

