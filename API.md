# SigmaOS API Documentation

## 1. Kernel Subsystem APIs

*   **Syscall Dispatcher**: Core API `syscall(number, args, capability)` handles process transitions.
*   **VFS (Virtual Filesystem)**: `VirtualFilesystem::read_inode`, `write_inode`.
*   **Capability Tokens**: `Capability::new()`, `has_permission()`.

## 2. Driver Interfaces

*   **Storage**: `SigmaFS` read/write block interfaces.
*   **Network**: Poll-based and interrupt-driven receive/transmit rings.
*   **GPU/Display**: FB/DRM inspired `DeviceContext`.

## 3. Package Management (SigmaPkg)

*   APIs for declarative configuration (`install_pkg`, `remove_pkg`, `rollback_transaction`).
*   Built-in resolution using SAT solvers.

## 4. Security API

*   `Pledge`: Restrict syscalls for the current thread.
*   `Unveil`: Expose specific VFS paths safely to a sandboxed process.
