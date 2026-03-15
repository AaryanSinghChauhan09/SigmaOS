# Generated class core: Architecture
from enum import Enum, auto

class Architecture(Enum):
    X86_64 = 'x86_64 (Intel/AMD)'
    ARM64 = 'AArch64 (Apple M-Series/Snapdragon)'
    RISCV = 'RISC-V (Open ISA)'
    WASM = 'WebAssembly Virtual ISA'
    QUANTUM = 'QPU Accelerated'
    IOT_EDGE = 'IoT Microcontroller (16MB+)'