"""True async filesystem I/O — no fake async, no GIL stalls."""

from typing import List

try:
    from _rapfiles import read_file_async, write_file_async  # type: ignore[import-not-found]
except ImportError:
    # Try alternative import path
    try:
        from rapfiles._rapfiles import read_file_async, write_file_async
    except ImportError:
        raise ImportError(
            "Could not import _rapfiles. Make sure rapfiles is built with maturin."
        )

__version__: str = "0.0.2"
__all__: List[str] = ["read_file_async", "write_file_async"]


# Convenience async functions
async def read_file(path: str) -> str:
    """Read a file asynchronously using true async I/O."""
    return await read_file_async(path)


async def write_file(path: str, contents: str) -> None:
    """Write a file asynchronously using true async I/O."""
    await write_file_async(path, contents)
