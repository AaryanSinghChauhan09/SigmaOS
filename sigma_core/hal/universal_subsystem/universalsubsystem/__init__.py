# Generated method: UniversalSubsystem.__init__
import os
import struct
from typing import Optional

class UniversalSubsystem:
    def __init__(self, kernel):
        self.kernel = kernel
        self.supported_formats = ['ELF', 'PE', 'MACHO']