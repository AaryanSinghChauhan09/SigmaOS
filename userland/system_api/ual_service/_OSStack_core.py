# Generated class core: OSStack
from enum import Enum
from dataclasses import dataclass
import uuid

class OSStack(Enum):
    WIN64 = 'Windows (x64/ARM64/PE)'
    DARWIN = 'macOS (Mach-O/Silicon/Intel)'
    BIONIC = 'Android (APK/AAB/Linux)'
    GNU = 'Linux (ELF/x86_64/ARM/Deb/Rpm)'
    WEB = 'Universal Web (Wasm/JS)'