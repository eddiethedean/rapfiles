🚀 RAP Update: rapfiles v0.2.0 is live

Phase 2 complete. We're building real async file I/O that actually delivers.

What's new in v0.2.0:

✅ Advanced file operations
- File manipulation: copy, move, rename, remove, hard links, symlinks
- Atomic operations: safe file writes and moves (never partial files)
- File locking: advisory locks (shared/exclusive) for coordination
- Batch operations: concurrent read/write/copy of multiple files

The technical wins:
- 188+ tests passing (up from 34+)
- All operations execute outside the Python GIL
- Zero Python thread pools (100% Rust + Tokio)
- Cross-platform support: Windows, macOS, Linux
- Python 3.8+ support (including Python 3.13 and 3.14)

What this means:
Need atomic writes for config files? File locking for coordination? Batch processing of thousands of files? rapfiles v0.2.0 has you covered. Still a drop-in replacement for aiofiles, but with actual async behavior—no fake async, no thread pool overhead.

The promise holds:
If it has the "rap" prefix, it's verified for real async behavior—no fake async, no thread pool overhead, no GIL stalls.

What's next:
- Phase 3 planning: streaming operations, memory-mapped files, advanced I/O patterns
- Continued validation: passing 100% of aiofiles test suite (working toward full compatibility)
- Performance benchmarks and comparisons with aiofiles

The foundation grows stronger. The journey continues.

Try it: https://github.com/eddiethedean/rapfiles

Feedback welcome. Contributions welcome.

#Python #AsyncIO #Rust #OpenSource #Performance
