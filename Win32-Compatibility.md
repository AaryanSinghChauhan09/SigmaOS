# Win32 Compatibility Layer

SigmaOS ships a lightweight Win32 compatibility layer (`runtime/compat/win32/`)
that allows running Windows PE32+ executables without a full Windows installation.
This is the SigmaOS equivalent of Wine — a cleanroom implementation, no Windows
source code used.

---

## Components

```
runtime/compat/win32/
├── sigma_pe_loader.rs      PE32+ ELF loader (MZ header → section mapping)
├── sigma_ntdll.rs          NT API shim (ntdll.dll replacement)
├── sigma_handle_table.rs   Win32 HANDLE ↔ SigmaOS fd/tid mapping
└── sigma_wine_loader.rs    High-level binary launcher
```

---

## PE Loader (`sigma_pe_loader.rs`)

Parses and maps Windows PE32+ (64-bit) executables:

```rust
// Load a PE32+ binary into memory at load_base
let image = PeLoader::load(pe_bytes, load_base)?;

println!("Entry:    0x{:x}", image.entry_point);
println!("Sections: {}", image.section_count);
println!("Imports:  {} DLLs", image.import_count);
```

### Features
- **PE32+ (x86-64)** — supports both EXE and DLL
- **Section mapping** — `.text` (r-x), `.data` (rw-), `.rdata` (r--) mapped with correct permissions
- **W^X enforcement** — sections cannot be both writable and executable (SigmaOS security policy)
- **Base relocations** — `.reloc` section processed when ASLR loads at non-preferred base
- **Import table parsing** — lists all DLL dependencies (resolved by the ntdll shim)
- **TLS callbacks** — Thread Local Storage initializers called on load

### Section Permissions (W^X)

```rust
// SigmaOS enforces W^X — a section trying to be both writable AND executable
// is rejected at load time:
if exec && write { return Err(PeError::W_XViolation); }
```

This is stricter than Windows itself, which allows RWX memory.

---

## NT API Shim (`sigma_ntdll.rs`)

Provides the NT functions that Win32 apps call at the bottom of every Windows
API call chain:

| NT Function | SigmaOS Equivalent |
|---|---|
| `NtAllocateVirtualMemory` | `sigma_mmap` with prot flags translation |
| `NtFreeVirtualMemory` | `sigma_munmap` |
| `NtCreateThread` | `sigma_thread_create` |
| `NtTerminateThread` | `sigma_thread_exit` |
| `NtDelayExecution` | `sigma_sleep_ms` |
| `NtQuerySystemTime` | `sigma_clock_ns` + epoch conversion |
| `RtlInitUnicodeString` | Inline (no syscall needed) |
| `RtlFreeUnicodeString` | `sigma_free` |
| `RtlCopyUnicodeString` | `sigma_alloc` + memcpy |
| `RtlAnsiStringToUnicodeString` | ASCII→UTF-16 conversion |

### Windows FILETIME ↔ SigmaOS clock

Windows uses 100-nanosecond intervals since **1601-01-01** (FILETIME).
SigmaOS uses nanoseconds since **Unix epoch** (1970-01-01).
The shim converts automatically:

```rust
const EPOCH_DIFF_100NS: u64 = 116_444_736_000_000_000;
*system_time = EPOCH_DIFF_100NS + sigma_clock_ns() / 100;
```

---

## Handle Table (`sigma_handle_table.rs`)

Windows uses opaque integer HANDLEs for all resources. The handle table maps
Win32 HANDLEs to SigmaOS native IDs:

```
Win32 HANDLE value = (table_index << 2)
                     ↕
sigma_handle_table entry:
  kind:  File | Thread | Process | Event | Mutex | Semaphore | ...
  data:  fd (file) | tid (thread) | event_id | mutex_id | ...
```

### Handle Types

| Kind | Win32 Source | SigmaOS data field |
|------|-------------|-------------------|
| `File` | `CreateFile` | SigmaOS file descriptor |
| `Thread` | `CreateThread` | thread ID |
| `Process` | `OpenProcess` | PID |
| `Event` | `CreateEvent` | event ID |
| `Mutex` | `CreateMutex` | mutex ID |
| `Semaphore` | `CreateSemaphore` | semaphore ID |
| `Section` | `CreateFileMapping` | shared memory ID |
| `Key` | `RegOpenKey` | registry key ID |

### Special Pseudo-Handles

```rust
pub const CURRENT_PROCESS: usize = !0usize;  // (HANDLE)-1
pub const CURRENT_THREAD:  usize = !1usize;  // (HANDLE)-2
// Always valid, don't need to be in the table
```

---

## Running a Win32 Binary

```bash
# Run a Windows .exe under SigmaOS
sigma-compat run notepad.exe
sigma-compat run setup.exe /S

# List loaded modules for a running Win32 process
sigma-compat modules <pid>

# Check if a binary is compatible
sigma-compat check myapp.exe
```

The launcher (`sigma_wine_loader.rs`):
1. Reads PE32+ header, validates it's 64-bit
2. Allocates memory at preferred base (or ASLR random base)
3. Maps all sections with `NtAllocateVirtualMemory`
4. Processes base relocations if base differs from preferred
5. Resolves imports — maps `ntdll.dll` calls to `sigma_ntdll.rs` shim
6. Calls TLS callbacks
7. Calls entry point with `argc/argv/envp`

---

## Security Considerations

Win32 compat processes run with:

```
sigma_pledge("stdio rpath wpath exec proc inet")
sigma_unveil("/tmp", "rwc")
sigma_unveil("~/.wine", "rwc")
```

They **cannot** access kernel internals or other processes' memory.
The PE loader enforces W^X — no RWX memory regions.

---

## Limitations (Phase A)

- ❌ 32-bit PE32 (x86) not supported — only PE32+ (x86-64)
- ❌ GUI (USER32/GDI32) not yet wired — console apps only
- ❌ COM/OLE not implemented
- ❌ Registry (`RegOpenKey` etc.) is a stub
- ❌ DirectX — Phase E
- ✅ Console I/O via `sigma_ntdll` → sigma-sh
- ✅ File I/O (`CreateFile`, `ReadFile`, `WriteFile`)
- ✅ Threading (`CreateThread`, `WaitForSingleObject` via event shim)
- ✅ Memory management (`VirtualAlloc`, `VirtualFree`)

---

*Source: `runtime/compat/win32/` · See also: [Windows-Linux-SigmaOS-Drivers](Windows-Linux-SigmaOS-Drivers) · [Linux Driver Compat](Linux-Driver-Compat)*
