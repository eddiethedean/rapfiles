"""Custom exception classes for rapfiles (aiofiles compatible)."""


class RAPFilesError(Exception):
    """Base exception for all rapfiles errors."""
    pass


class RAPFilesIOError(RAPFilesError, IOError):
    """I/O error in rapfiles operations."""
    pass


class RAPFilesOSError(RAPFilesError, OSError):
    """OS error in rapfiles operations."""
    pass


class RAPFilesValueError(RAPFilesError, ValueError):
    """Value error in rapfiles operations."""
    pass


class RAPFilesTypeError(RAPFilesError, TypeError):
    """Type error in rapfiles operations."""
    pass


class RAPFilesFileNotFoundError(RAPFilesIOError, FileNotFoundError):
    """File not found error."""
    pass


class RAPFilesPermissionError(RAPFilesOSError, PermissionError):
    """Permission denied error."""
    pass


class RAPFilesIsADirectoryError(RAPFilesOSError, IsADirectoryError):
    """Operation on directory when file expected."""
    pass


class RAPFilesNotADirectoryError(RAPFilesOSError, NotADirectoryError):
    """Operation on file when directory expected."""
    pass


__all__ = [
    "RAPFilesError",
    "RAPFilesIOError",
    "RAPFilesOSError",
    "RAPFilesValueError",
    "RAPFilesTypeError",
    "RAPFilesFileNotFoundError",
    "RAPFilesPermissionError",
    "RAPFilesIsADirectoryError",
    "RAPFilesNotADirectoryError",
]
