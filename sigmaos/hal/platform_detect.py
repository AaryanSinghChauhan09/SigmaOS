"""
SigmaOS Hardware Abstraction Layer (HAL)
Centralizes all OS/platform detection to eliminate scattered standard library usage.
"""
import platform
import sys

# Immutable Platform Constants
SYSTEM = platform.system().lower()
IS_WINDOWS = SYSTEM == "windows"
IS_LINUX = SYSTEM == "linux"
IS_MAC = SYSTEM == "darwin"

ARCH = platform.machine().lower()
IS_X86_64 = ARCH in ["x86_64", "amd64"]
IS_ARM = ARCH in ["arm64", "aarch64"]

def get_platform_info() -> dict:
    return {
        "os": SYSTEM,
        "arch": ARCH,
        "python_version": sys.version.split()[0]
    }
