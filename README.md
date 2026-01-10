# rapfiles

**True async filesystem I/O — no fake async, no GIL stalls.**

---

## Why `rap*`?

Packages prefixed with **`rap`** stand for **Real Async Python**. Unlike many libraries that merely wrap blocking I/O in `async` syntax, `rap*` packages guarantee that all I/O work is executed **outside the Python GIL** using native runtimes (primarily Rust). This means event loops are never stalled by hidden thread pools, blocking syscalls, or cooperative yielding tricks. If a `rap*` API is `async`, it is *structurally non-blocking by design*, not by convention. The `rap` prefix is a contract: measurable concurrency, real parallelism, and verifiable async behavior under load.

See the [rap-manifesto](https://github.com/rap-project/rap-manifesto) for philosophy and guarantees.

---

## What this package provides

- True async file reads and writes
- Native Rust-backed execution (Tokio)
- Zero Python thread pools
- Event-loop-safe concurrency under load
- GIL-independent I/O operations

---

## Installation

```bash
pip install rapfiles
```

---

## Usage

```python
import asyncio
from rapfiles import read_file, write_file

async def main():
    # Read file asynchronously (true async, GIL-independent)
    content = await read_file("example.txt")
    
    # Write file asynchronously (true async, GIL-independent)
    await write_file("output.txt", content)

asyncio.run(main())
```

---

## Benchmarks

This package passes the Fake Async Detector. Benchmarks are available in the [rap-bench](https://github.com/rap-project/rap-bench) repository.

---

## Non-Goals

- Not a drop-in replacement for `asyncio` file I/O
- Not compatible with all filesystem operations (yet)
- Not designed for synchronous use cases

---

## License

MIT
