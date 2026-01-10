"""True async filesystem I/O — no fake async, no GIL stalls."""

try:
    from _rapfiles import read_file_async, write_file_async
except ImportError:
    # Try alternative import path
    try:
        from rapfiles._rapfiles import read_file_async, write_file_async
    except ImportError:
        raise ImportError(
            "Could not import _rapfiles. Make sure rapfiles is built with maturin."
        )

__version__ = "0.0.1"
__all__ = ["read_file_async", "write_file_async"]


# Convenience async functions
async def read_file(path: str) -> str:
    """Read a file asynchronously using true async I/O."""
    return await read_file_async(path)


async def write_file(path: str, contents: str) -> None:
    """Write a file asynchronously using true async I/O."""
    await write_file_async(path, contents)
