# SigmaOS Libc

Provides a zero-dependency, bare-metal C runtime library (libc equivalent) for the SigmaOS microkernel, systems services, and userland applications.

## Standard Primitives

- **Memory Operators**:
  - `sigma_memcpy`: Silicon-optimized memory copy.
  - `sigma_memset`: Quick block memory filling.

- **String Formatting & I/O**:
  - `sigma_printf`: Standard string formatter printing to COM1 serial interface.
  - `sigma_strlen`, `sigma_strcmp`, `sigma_strncpy`: Classic string manipulators.

- **Utility & Integrity**:
  - `sigma_atoi`: Basic numeric string converter.
  - `sigma_exit`: Clean CPU execution halting and resource eviction.
  - `sigma_crc32`: Polynomial redundancy checks for files and packets.
