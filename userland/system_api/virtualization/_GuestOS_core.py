# Generated class core: GuestOS
import time
import uuid
from dataclasses import dataclass
from enum import Enum, auto

class GuestOS(Enum):
    WINDOWS = 'Windows (Win32/NT)'
    MACOS = 'macOS (Cocoa/Mach)'
    LINUX = 'Linux (ELF/POSIX)'
    ANDROID = 'Android (APK/ART)'
    WASM = 'WebAssembly (WASI)'
    SIGMA = 'SigmaOS (Native)'