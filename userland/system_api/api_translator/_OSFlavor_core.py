# Generated class core: OSFlavor
from enum import Enum
import time
import uuid

class OSFlavor(Enum):
    WIN32 = 'Windows (x64/ARM)'
    MACOS = 'macOS (Cocoa/Mach)'
    ANDROID = 'Android (Bionic/Linux)'
    LINUX = 'Linux (GNU/POSIX)'
    SIGMA = 'SigmaOS Native'