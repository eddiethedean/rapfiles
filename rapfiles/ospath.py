"""Path operations module (aiofiles.ospath compatible)."""

import os
from pathlib import Path
from typing import Union

# Re-export common path operations for aiofiles compatibility
# These are synchronous operations that work with paths (not file I/O)

def exists(path: Union[str, bytes, Path]) -> bool:
    """Check if a path exists (synchronous)."""
    return os.path.exists(path)


def isfile(path: Union[str, bytes, Path]) -> bool:
    """Check if a path is a file (synchronous)."""
    return os.path.isfile(path)


def isdir(path: Union[str, bytes, Path]) -> bool:
    """Check if a path is a directory (synchronous)."""
    return os.path.isdir(path)


def getsize(path: Union[str, bytes, Path]) -> int:
    """Get the size of a file (synchronous)."""
    return os.path.getsize(path)


def join(*paths: Union[str, bytes, Path]) -> str:
    """Join path components (synchronous)."""
    return os.path.join(*paths)


def normpath(path: Union[str, bytes, Path]) -> str:
    """Normalize a path (synchronous)."""
    return os.path.normpath(path)


def abspath(path: Union[str, bytes, Path]) -> str:
    """Get absolute path (synchronous)."""
    return os.path.abspath(path)


def dirname(path: Union[str, bytes, Path]) -> str:
    """Get directory name (synchronous)."""
    return os.path.dirname(path)


def basename(path: Union[str, bytes, Path]) -> str:
    """Get base name (synchronous)."""
    return os.path.basename(path)


def splitext(path: Union[str, bytes, Path]) -> tuple[str, str]:
    """Split path into (root, ext) (synchronous)."""
    return os.path.splitext(path)


def split(path: Union[str, bytes, Path]) -> tuple[str, str]:
    """Split path into (head, tail) (synchronous)."""
    return os.path.split(path)


__all__ = [
    "exists",
    "isfile",
    "isdir",
    "getsize",
    "join",
    "normpath",
    "abspath",
    "dirname",
    "basename",
    "splitext",
    "split",
]
