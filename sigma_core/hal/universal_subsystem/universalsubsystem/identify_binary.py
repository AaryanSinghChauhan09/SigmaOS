# Generated method: UniversalSubsystem.identify_binary
import os
import struct
from typing import Optional

class UniversalSubsystem:
    def identify_binary(self, file_path: str) -> Optional[str]:
        """Reads file headers to detect the binary format."""
        if not os.path.exists(file_path):
            return None
        with open(file_path, 'rb') as f:
            magic = f.read(4)
            if magic.startswith(b'\x7fELF'):
                return 'ELF'
            if magic.startswith(b'MZ'):
                return 'PE'
            if magic in [b'\xce\xfa\xed\xfe', b'\xcf\xfa\\ed\xfe']:
                return 'MACHO'
        return None