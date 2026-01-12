"""True async filesystem I/O — no fake async, no GIL stalls."""

from typing import List, Optional, Union, Any

try:
    from _rapfiles import (  # type: ignore[import-not-found]
        read_file_async,
        write_file_async,
        read_file_bytes_async,
        write_file_bytes_async,
        append_file_async,
        open_file,
        AsyncFile,
        create_dir_async,
        create_dir_all_async,
        remove_dir_async,
        remove_dir_all_async,
        list_dir_async,
        exists_async,
        is_file_async,
        is_dir_async,
        stat_async,
        metadata_async,
        FileMetadata,
        walk_dir_async,
    )
except ImportError:
    # Try alternative import path
    try:
        from rapfiles._rapfiles import (
            read_file_async,
            write_file_async,
            read_file_bytes_async,
            write_file_bytes_async,
            append_file_async,
            open_file,
            AsyncFile,
            create_dir_async,
            create_dir_all_async,
            remove_dir_async,
            remove_dir_all_async,
            list_dir_async,
            exists_async,
            is_file_async,
            is_dir_async,
        )
    except ImportError:
        raise ImportError(
            "Could not import _rapfiles. Make sure rapfiles is built with maturin."
        )

__version__: str = "0.0.2"

# Import ospath module for aiofiles compatibility
from rapfiles import ospath  # noqa: F401
__all__: List[str] = [
    # File operations
    "read_file_async",
    "write_file_async",
    "read_file_bytes_async",
    "write_file_bytes_async",
    "append_file_async",
    "read_file",
    "write_file",
    "read_file_bytes",
    "write_file_bytes",
    "append_file",
    # File handles
    "open",
    "open_file",
    "AsyncFile",
    # Directory operations
    "create_dir",
    "create_dir_all",
    "remove_dir",
    "remove_dir_all",
    "list_dir",
    "exists",
    "is_file",
    "is_dir",
    # Metadata operations
    "stat",
    "metadata",
    "FileMetadata",
    # Directory traversal
    "walk_dir",
]


# Convenience async functions
async def read_file(path: str) -> str:
    """Read a file asynchronously using true async I/O."""
    return await read_file_async(path)


async def write_file(path: str, contents: str) -> None:
    """Write a file asynchronously using true async I/O."""
    await write_file_async(path, contents)


async def read_file_bytes(path: str) -> bytes:
    """Read a file asynchronously as bytes."""
    return await read_file_bytes_async(path)


async def write_file_bytes(path: str, contents: bytes) -> None:
    """Write bytes to a file asynchronously."""
    await write_file_bytes_async(path, contents)


async def append_file(path: str, contents: str) -> None:
    """Append to a file asynchronously."""
    await append_file_async(path, contents)


# Directory operations
async def create_dir(path: str) -> None:
    """Create a directory asynchronously."""
    await create_dir_async(path)


async def create_dir_all(path: str) -> None:
    """Create a directory and all parent directories asynchronously."""
    await create_dir_all_async(path)


async def remove_dir(path: str) -> None:
    """Remove an empty directory asynchronously."""
    await remove_dir_async(path)


async def remove_dir_all(path: str) -> None:
    """Remove a directory and all its contents asynchronously."""
    await remove_dir_all_async(path)


async def list_dir(path: str) -> list[str]:
    """List directory contents asynchronously."""
    return await list_dir_async(path)


async def exists(path: str) -> bool:
    """Check if a path exists asynchronously."""
    return await exists_async(path)


async def is_file(path: str) -> bool:
    """Check if a path is a file asynchronously."""
    return await is_file_async(path)


async def is_dir(path: str) -> bool:
    """Check if a path is a directory asynchronously."""
    return await is_dir_async(path)


# Metadata operations
async def stat(path: str) -> "FileMetadata":
    """Get file statistics asynchronously."""
    return await stat_async(path)


async def metadata(path: str) -> "FileMetadata":
    """Get file metadata asynchronously (alias for stat)."""
    return await metadata_async(path)


# Directory traversal
async def walk_dir(path: str) -> list[tuple[str, bool]]:
    """
    Recursively walk a directory asynchronously.
    
    Args:
        path: Directory path to walk
        
    Returns:
        List of (path, is_file) tuples for all files and directories found
    """
    return await walk_dir_async(path)


# aiofiles.open() compatible function
async def open(
    file: Union[str, bytes],
    mode: str = "r",
    buffering: int = -1,
    encoding: Optional[str] = None,
    errors: Optional[str] = None,
    newline: Optional[str] = None,
    closefd: bool = True,
    opener: Optional[Any] = None,
) -> AsyncFile:
    """
    Open a file asynchronously (aiofiles.open() compatible).
    
    Args:
        file: Path to the file (str or bytes)
        mode: File mode (r, r+, w, w+, a, a+, rb, rb+, wb, wb+, ab, ab+)
        buffering: Buffer size (not yet implemented, accepted for compatibility)
        encoding: Text encoding (not yet implemented, accepted for compatibility)
        errors: Error handling (not yet implemented, accepted for compatibility)
        newline: Newline handling (not yet implemented, accepted for compatibility)
        closefd: Close file descriptor (not yet implemented, accepted for compatibility)
        opener: Custom opener (not yet implemented, accepted for compatibility)
    
    Returns:
        AsyncFile: An async file handle that can be used with async context managers
        
    Example:
        ```python
        async with open("file.txt", "r") as f:
            content = await f.read()
        ```
    """
    if isinstance(file, bytes):
        file = file.decode("utf-8")
    return await open_file(file, mode, buffering, encoding, errors, newline, closefd, opener)
