"""Test rapfiles async functionality."""

import asyncio
import tempfile
import os

from rapfiles import read_file, write_file


async def test_rapfiles():
    """Test basic async file operations."""
    # Create a temporary file
    with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
        test_file = f.name
        f.write("test content")
    
    try:
        # Test async read
        print("Testing async read...")
        content = await read_file(test_file)
        print(f"Read content: {content}")
        assert content == "test content", f"Expected 'test content', got '{content}'"
        
        # Test async write
        print("Testing async write...")
        await write_file(test_file, "new content")
        
        # Verify write
        print("Verifying write...")
        content = await read_file(test_file)
        print(f"Read new content: {content}")
        assert content == "new content", f"Expected 'new content', got '{content}'"
        
        print("All tests passed!")
    finally:
        # Cleanup
        if os.path.exists(test_file):
            os.unlink(test_file)


if __name__ == "__main__":
    asyncio.run(test_rapfiles())

