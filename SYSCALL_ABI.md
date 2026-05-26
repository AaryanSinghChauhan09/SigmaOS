# SigmaOS Syscall Application Binary Interface (ABI)

SigmaOS rejects POSIX in favor of a sovereign, deterministic, and capability-driven syscall interface. This document outlines the strict Syscall ABI required for userspace shards to communicate with the sovereign kernel.

## Calling Convention (x86_64)

To invoke a syscall in SigmaOS, shards must use the `syscall` instruction. The registers are utilized as follows:

| Register | Purpose |
|----------|---------|
| `RAX`    | Syscall Number (See `SYSCALLS.md`) |
| `RDI`    | Argument 1 |
| `RSI`    | Argument 2 |
| `RDX`    | Argument 3 |
| `R10`    | Argument 4 |
| `R8`     | Argument 5 |
| `R9`     | Argument 6 |

### Kernel Return Values

After the `syscall` instruction completes, the kernel places the return value in the `RAX` register.

- **Success**: A non-negative integer (e.g., `0` for success, or a positive handle ID/file descriptor).
- **Error**: A negative integer mapping to a specific Sovereign Error Code (e.g., `-1` for `E_CAP_DENIED`, `-2` for `E_INVALID_ARG`).

*Note: The `RCX` and `R11` registers are clobbered by the `syscall` instruction itself and must not be relied upon to preserve state.*

## Data Structure Alignment

To ensure compatibility across language runtimes and prevent memory fragmentation:
- All structures passed via pointer (e.g., `sigma_stat` structs) must be `8-byte` aligned.
- Bitfields are strictly prohibited in syscall structs to avoid compiler-specific ABI variations.
- Pointers within structs must be absolute virtual addresses valid within the calling shard's memory horizon.

## Cryptographic Sealing

Certain sensitive syscalls (like `sigma_ipc_call` or `sigma_send`) require payloads to be sealed using the Shard's assigned PQC Dilithium key. In these cases:
- `RDI` points to the sealed payload envelope.
- `RSI` contains the size of the envelope.
- `RDX` points to the detached cryptographic signature.
