"""Type stubs for _rapfiles Rust extension module."""

from typing import Coroutine, Any, Optional, Union

def read_file_async(path: str) -> Coroutine[Any, Any, str]: ...
def write_file_async(path: str, contents: str) -> Coroutine[Any, Any, None]: ...

def open_file(
    path: str,
    mode: str = "r",
    buffering: int = -1,
    encoding: Optional[str] = None,
    errors: Optional[str] = None,
    newline: Optional[str] = None,
    closefd: bool = True,
    opener: Optional[Any] = None,
) -> Coroutine[Any, Any, "AsyncFile"]: ...

class AsyncFile:
    """Async file handle for true async I/O operations."""
    
    def __init__(
        self,
        path: str,
        mode: str = "r",
        buffering: int = -1,
        encoding: Optional[str] = None,
        errors: Optional[str] = None,
        newline: Optional[str] = None,
        closefd: bool = True,
        opener: Optional[Any] = None,
    ) -> None: ...
    
    def read(self, size: int = -1) -> Coroutine[Any, Any, Union[str, bytes]]: ...
    def write(self, data: Union[str, bytes]) -> Coroutine[Any, Any, int]: ...
    def readline(self, size: int = -1) -> Coroutine[Any, Any, Union[str, bytes]]: ...
    def readlines(self, hint: int = -1) -> Coroutine[Any, Any, Union[list[str], list[bytes]]]: ...
    def seek(self, offset: int, whence: int = 0) -> Coroutine[Any, Any, int]: ...
    def tell(self) -> Coroutine[Any, Any, int]: ...
    def close(self) -> Coroutine[Any, Any, None]: ...
    
    def __aenter__(self) -> Coroutine[Any, Any, "AsyncFile"]: ...
    def __aexit__(
        self,
        exc_type: Optional[type],
        exc_val: Optional[BaseException],
        exc_tb: Optional[Any],
    ) -> Coroutine[Any, Any, bool]: ...
