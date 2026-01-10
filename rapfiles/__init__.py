"""True async filesystem I/O — no fake async, no GIL stalls."""

from rapfiles._rapfiles import read_file_async, write_file_async

__version__ = "0.0.1"
__all__ = ["read_file_async", "write_file_async"]


# Convenience async functions
async def read_file(path: str) -> str:
    """Read a file asynchronously using true async I/O."""
    return await read_file_async(path)


async def write_file(path: str, contents: str) -> None:
    """Write a file asynchronously using true async I/O."""
    await write_file_async(path, contents)

