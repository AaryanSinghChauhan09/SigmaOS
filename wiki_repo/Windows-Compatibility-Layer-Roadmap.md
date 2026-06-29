# SigmaOS — Windows Compatibility Layer Roadmap

A phased, file-level engineering plan to give SigmaOS the ability to run
unmodified Windows PE/Win32 binaries, with the architecture, dependency chain,
and exact implementation targets for each stage.

---

## Why This Matters

The biggest adoption barrier for any non-Windows OS is not the kernel — it is
the **18 billion lines of installed Win32 application code** that users depend on.
Wine took 30 years. Proton took 6. SigmaOS will build on their lessons with a
cleaner architecture.

**What already exists in the repo:**
- `runtime/containers/sigma_linux_compat.cpp` — ELF64 loader + Linux syscall translator (15 syscalls mapped)
- `userland/compat/POSIXShim.cpp` — POSIX open/read/write/close/fork/execve shim
- `userland/compat/sigma_proton_bridge.cpp` — Proton-style syscall trap + `mapDxvkSurface()` stub

**What needs to be built:** everything Win32.

---

## Architecture Overview

```
Windows .exe / .dll (PE32+)
        │
        ▼
┌───────────────────────────────────────────────────────────┐
│              sigma-wine  (Win32 Compatibility Layer)       │
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌─────────────────┐ │
│  │  PE Loader   │  │ NT Syscall   │  │  Win32 API DLLs │ │
│  │  (sigma-pe)  │  │  Translator  │  │  (sigma-ntdll   │ │
│  │              │  │  (NtXxx →    │  │   sigma-kernel32│ │
│  │  PE32+ parse │  │   sigma-sys) │  │   sigma-user32) │ │
│  └──────────────┘  └──────────────┘  └─────────────────┘ │
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌─────────────────┐ │
│  │  Registry    │  │  COM/OLE     │  │  D3D/DXVK       │ │
│  │  sigma-reg   │  │  sigma-com   │  │  → Vulkan bridge│ │
│  └──────────────┘  └──────────────┘  └─────────────────┘ │
└───────────────────────────────────────────────────────────┘
        │
        ▼
SigmaOS kernel (sigma-syscall ABI)
        │
        ▼
SigmaOS hardware (SDF drivers)
```

---

## Dependency Chain (What Blocks What)

```
SigmaOS kernel boots (Phase 0)
    └── sigma-syscall ABI stable (30 calls)
        └── sigma-vmm address space management
            └── PE Loader (sigma-pe) — Stage 1
                └── NT syscall translator (sigma-ntdll) — Stage 2
                    └── sigma-kernel32 / sigma-user32 — Stage 3
                        └── sigma-gdi32 / sigma-comctl32 — Stage 4
                            └── sigma-com (COM/OLE/ATL) — Stage 5
                                └── DXVK bridge (D3D → Vulkan) — Stage 6
                                    └── sigma-msvcrt (CRT runtime) — Stage 6
                                        └── Full Win32 app compatibility — Stage 7
```

The kernel **must** boot before any of this is meaningful. All compat work is
blocked until Phase 0 completes (`sigma_sched`, `sigma_mm`, `sigma_syscall_dispatch`).

---

## Stage 0 — Prerequisites (Before Any Win32 Work)

These are not compat-layer tasks — they are kernel tasks that unblock everything.

| Task | File | Status | 
| ------ | ------ | -------- | 
| Kernel scheduler (MLFQ) | `kernel/core/sigma_sched.cpp` | `[ ]` | 
| Memory manager (buddy+slab+VMM) | `kernel/core/sigma_mm.cpp` | `[ ]` | 
| Syscall dispatch (30 calls) | `kernel/core/sigma_syscall_dispatch.cpp` | `[ ]` | 
| Virtual memory manager (mmap/munmap/mprotect) | `kernel/mm/sigma_vmm.cpp` | `[ ]` | 
| Process creation (fork/exec model) | `kernel/core/sigma_proc.cpp` | `[ ]` | 
| Dynamic linker foundation | `userland/ldso/sigma_ldso.cpp` | `[ ]` | 

**Exit gate:** `sigma-compat exec hello.exe` reaches the PE entry point in QEMU.

---

## Stage 1 — PE Loader (`sigma-pe`)

### What it does
Parses and loads Windows PE32+ executable images into a SigmaOS address space,
the same way `sigma_linux_compat.cpp` handles ELF64.

### Files to create
```
runtime/compat/win32/sigma_pe_loader.cpp     ← PE32+ parser + segment mapper
runtime/compat/win32/sigma_pe_loader.h
include/compat/sigma_pe_types.h              ← IMAGE_DOS_HEADER, IMAGE_NT_HEADERS, etc.
```

### Key structures to implement
```cpp
// IMAGE_DOS_HEADER — 64 bytes
// IMAGE_NT_HEADERS64 — signature + FileHeader + OptionalHeader
// IMAGE_SECTION_HEADER — per-section: VirtualAddress, SizeOfRawData, Characteristics
// IMAGE_IMPORT_DESCRIPTOR — import table for DLL resolution
// IMAGE_EXPORT_DIRECTORY — export table for sigma-ntdll stubs
// IMAGE_THUNK_DATA — IAT / INT entries
// IMAGE_TLS_DIRECTORY — Thread Local Storage
// IMAGE_LOAD_CONFIG_DIRECTORY — SEH, CFGuard, ASLR metadata
```

### Loader steps
```
1. Validate MZ magic (0x4D5A) + PE signature (0x50450000)
2. Parse IMAGE_FILE_HEADER → machine type, section count, characteristics
3. Parse IMAGE_OPTIONAL_HEADER64 → ImageBase, AddressOfEntryPoint, SizeOfImage
4. Map PE sections via sigma_vmm_map_region():
   .text  → R-X
   .rdata → R--
   .data  → RW-
   .bss   → RW- (zero-fill)
5. Apply base relocations (IMAGE_BASE_RELOCATION) if loaded != preferred base
6. Resolve imports (IMAGE_IMPORT_DESCRIPTOR) → fill IAT with sigma-ntdll stubs
7. Handle TLS callbacks (IMAGE_TLS_DIRECTORY) before entry point
8. Call entry point: DllMain(hmod, DLL_PROCESS_ATTACH, NULL) or WinMain
```

### Status: `[ ]` Not started

---

## Stage 2 — NT Syscall Translator (`sigma-ntdll`)

### What it does
The NT native API (`NtXxx` / `ZwXxx` functions in `ntdll.dll`) is the actual
syscall boundary of Windows — Win32 API sits on top of it. Translating NT calls
to sigma-syscall is the highest-leverage work in the entire compat stack.

### Files to create
```
runtime/compat/win32/sigma_ntdll.cpp         ← NT native API implementation
runtime/compat/win32/sigma_nt_syscall_table.cpp ← NT→sigma syscall number map
include/compat/sigma_nt_types.h              ← NTSTATUS, UNICODE_STRING, OBJECT_ATTRIBUTES, etc.
include/compat/sigma_nt_syscalls.h           ← NtXxx function declarations
```

### NT→SigmaOS syscall mapping (priority order)

| NT syscall | Maps to | Notes | 
| ------------ | --------- | ------- | 
| `NtReadFile` | `sigma_sys_read` | Handle → fd translation | 
| `NtWriteFile` | `sigma_sys_write` | Handle → fd | 
| `NtCreateFile` | `sigma_sys_open` | ObjectAttributes path extraction | 
| `NtClose` | `sigma_sys_close` | Handle table | 
| `NtAllocateVirtualMemory` | `sigma_sys_mmap` | MEM_COMMIT / MEM_RESERVE | 
| `NtFreeVirtualMemory` | `sigma_sys_munmap` | | 
| `NtProtectVirtualMemory` | `sigma_sys_mprotect` | PAGE_* → PROT_* | 
| `NtCreateProcess` | `sigma_sys_fork` + `exec` | | 
| `NtCreateThread` | `sigma_sys_thread_create` | TEB setup | 
| `NtTerminateProcess` | `sigma_sys_exit` | | 
| `NtTerminateThread` | `sigma_sys_thread_exit` | | 
| `NtQuerySystemInformation` | sigma-dna probe | SystemBasicInfo etc. | 
| `NtQueryInformationProcess` | sigma_proc_query | PEB pointer | 
| `NtQueryInformationThread` | sigma_thread_query | TEB pointer | 
| `NtSetInformationThread` | sigma_thread_set | ThreadBasicInfo | 
| `NtCreateSection` | `sigma_sys_mmap` | File-backed mapping | 
| `NtMapViewOfSection` | `sigma_sys_mmap` | | 
| `NtUnmapViewOfSection` | `sigma_sys_munmap` | | 
| `NtCreateKey` | sigma-reg create | Registry | 
| `NtOpenKey` | sigma-reg open | Registry | 
| `NtQueryValueKey` | sigma-reg query | Registry | 
| `NtSetValueKey` | sigma-reg set | Registry | 
| `NtCreateMutant` | sigma_mutex_create | | 
| `NtCreateEvent` | sigma_event_create | | 
| `NtWaitForSingleObject` | sigma_wait | Handles → sigma-bus | 
| `NtWaitForMultipleObjects` | sigma_wait_multi | | 
| `NtDelayExecution` | sigma_nanosleep | | 
| `NtQueryPerformanceCounter` | sigma_tsc_read | RDTSC-backed | 
| `NtGetTickCount64` | sigma_uptime_ms | | 
| `RtlAllocateHeap` | sigma_slab_alloc | NT heap → sigma slab | 
| `RtlFreeHeap` | sigma_slab_free | | 

### Handle table
Windows uses kernel handles (integers) for all objects — files, threads, events,
sections, keys. A **sigma-handle-table** must map NT handles to sigma objects:

```cpp
// runtime/compat/win32/sigma_handle_table.cpp
struct SigmaHandle {
    enum Type { FILE, THREAD, PROCESS, EVENT, MUTEX, SECTION, KEY, PIPE } type;
    union {
        sigma_fd_t   fd;          // Type::FILE
        sigma_tid_t  tid;         // Type::THREAD
        sigma_pid_t  pid;         // Type::PROCESS
        sigma_evid_t event_id;    // Type::EVENT
        sigma_mid_t  mutex_id;    // Type::MUTEX
        sigma_mid_t  section_id;  // Type::SECTION
        sigma_reg_key_t reg_key;  // Type::KEY
    };
};
// Per-process handle table: HANDLE (u32) → SigmaHandle
```

### PEB / TEB stubs
Win32 apps expect `fs:[0x18]` (TEB) and `gs:[0x60]` (PEB on x86-64) to be valid:

```cpp
// Minimum PEB fields that apps probe at startup
struct SigmaPEB {
    BYTE  BeingDebugged;        // offset 0x02 — set to 0
    PVOID ImageBaseAddress;     // offset 0x10
    PVOID Ldr;                  // offset 0x18 — module list
    PVOID ProcessParameters;    // offset 0x20 — RTL_USER_PROCESS_PARAMETERS
    ULONG OSMajorVersion;       // offset 0xA4 — report 10
    ULONG OSMinorVersion;       // offset 0xA8 — report 0
    ULONG OSBuildNumber;        // offset 0xAC — report 19041
};
```

### Status: `[ ]` Not started

---

## Stage 3 — Win32 API DLLs (`sigma-kernel32`, `sigma-user32`)

### sigma-kernel32

The most-imported DLL in Windows. Wraps NT calls with friendlier semantics.
Implemented as a native SigmaOS shared library that exports the `kernel32.dll` symbol table.

```
runtime/compat/win32/kernel32/sigma_kernel32.cpp
runtime/compat/win32/kernel32/sigma_kernel32_file.cpp      ← CreateFile, ReadFile, WriteFile
runtime/compat/win32/kernel32/sigma_kernel32_process.cpp   ← CreateProcess, ExitProcess
runtime/compat/win32/kernel32/sigma_kernel32_memory.cpp    ← VirtualAlloc, HeapAlloc
runtime/compat/win32/kernel32/sigma_kernel32_sync.cpp      ← CreateMutex, WaitForSingleObject
runtime/compat/win32/kernel32/sigma_kernel32_thread.cpp    ← CreateThread, TlsAlloc
runtime/compat/win32/kernel32/sigma_kernel32_time.cpp      ← GetTickCount, QueryPerformanceCounter
runtime/compat/win32/kernel32/sigma_kernel32_console.cpp   ← GetStdHandle, WriteConsoleA/W
runtime/compat/win32/kernel32/sigma_kernel32_module.cpp    ← LoadLibrary, GetProcAddress
runtime/compat/win32/kernel32/sigma_kernel32_error.cpp     ← GetLastError, SetLastError
```

**Priority functions (cover 90% of CLI apps):**

| Function | sigma-ntdll call | Notes | 
| ---------- | ----------------- | ------- | 
| `CreateFileA/W` | `NtCreateFile` | Path → ObjectAttributes conversion | 
| `ReadFile` | `NtReadFile` | Overlapped → sync wrapper | 
| `WriteFile` | `NtWriteFile` | | 
| `CloseHandle` | `NtClose` | | 
| `GetStdHandle` | handle table | stdin=0, stdout=1, stderr=2 | 
| `WriteConsoleA` | `NtWriteFile(stdout)` | | 
| `VirtualAlloc` | `NtAllocateVirtualMemory` | | 
| `VirtualFree` | `NtFreeVirtualMemory` | | 
| `HeapAlloc` | `RtlAllocateHeap` | | 
| `HeapFree` | `RtlFreeHeap` | | 
| `CreateProcess` | `NtCreateProcess` | lpCommandLine split | 
| `ExitProcess` | `NtTerminateProcess` | | 
| `GetModuleHandleA/W` | PEB Ldr walk | | 
| `LoadLibraryA/W` | sigma-ldso load | | 
| `GetProcAddress` | PE export table scan | | 
| `CreateThread` | `NtCreateThread` | | 
| `WaitForSingleObject` | `NtWaitForSingleObject` | | 
| `GetLastError` | TEB.LastError | | 
| `SetLastError` | TEB.LastError | | 
| `GetTickCount64` | sigma_uptime_ms | | 
| `QueryPerformanceCounter` | RDTSC | | 
| `MultiByteToWideChar` | sigma-locale UTF-8↔UTF-16 | | 
| `WideCharToMultiByte` | sigma-locale | | 

### sigma-user32

Handles window management, message pumps, and input. Backed by Zenith compositor.

```
runtime/compat/win32/user32/sigma_user32.cpp
runtime/compat/win32/user32/sigma_user32_window.cpp   ← CreateWindow, ShowWindow, SetWindowPos
runtime/compat/win32/user32/sigma_user32_msg.cpp      ← GetMessage, DispatchMessage, PostMessage
runtime/compat/win32/user32/sigma_user32_paint.cpp    ← BeginPaint, EndPaint, InvalidateRect
runtime/compat/win32/user32/sigma_user32_input.cpp    ← GetKeyState, GetCursorPos, mouse events
runtime/compat/win32/user32/sigma_user32_dialog.cpp   ← MessageBox, DialogBox, common controls
runtime/compat/win32/user32/sigma_user32_dc.cpp       ← GetDC, ReleaseDC (HDC → Vulkan surface)
```

**Win32 HWND → Zenith surface mapping:**
```
CreateWindow(...)
    → sigma_user32 allocates HWND slot in window table
        → sigma_compositor_create_surface(width, height)
            → Zenith compositor creates Vulkan swapchain
                → Returns Vulkan surface handle stored in HWND record
```

**Message pump → sigma-display event loop:**
```
GetMessage(MSG*)
    → sigma_display_poll_event()
        → maps sigma_key_event → WM_KEYDOWN/WM_KEYUP
        → maps sigma_mouse_event → WM_MOUSEMOVE/WM_LBUTTONDOWN
        → maps sigma_resize_event → WM_SIZE
```

### Status: `[ ]` Not started

---

## Stage 4 — GDI, Shell, Common Controls (`sigma-gdi32`, `sigma-shell32`)

### sigma-gdi32 (Graphics Device Interface)

GDI is the legacy 2D drawing API. Many Win32 apps use it for basic rendering.
Rather than implementing GDI in software, sigma-gdi32 routes all drawing calls
through Vulkan compute shaders via the Zenith compositor.

```
runtime/compat/win32/gdi32/sigma_gdi32.cpp
runtime/compat/win32/gdi32/sigma_gdi32_draw.cpp    ← TextOut, Rectangle, Ellipse, BitBlt
runtime/compat/win32/gdi32/sigma_gdi32_font.cpp    ← CreateFont, SelectObject (→ HarfBuzz)
runtime/compat/win32/gdi32/sigma_gdi32_bitmap.cpp  ← CreateBitmap, StretchBlt, DIBits
runtime/compat/win32/gdi32/sigma_gdi32_dc.cpp      ← CreateDC, DeleteDC, HDC→Vulkan
```

**HDC → Vulkan command buffer mapping:**
```
GDI HDC (Device Context)
    → sigma_gdi32 maintains HDC → Vulkan command buffer table
        → TextOut(hdc, x, y, text) → HarfBuzz shape → FreeType2 render → upload glyph atlas → Vulkan draw
        → BitBlt(hdc_dst, ..., hdc_src, ...) → Vulkan image copy
        → Rectangle(hdc, ...) → Vulkan filled rect pipeline
```

### sigma-shell32

Required for apps that use Explorer shell integration.

```
runtime/compat/win32/shell32/sigma_shell32.cpp
    ← ShellExecute (→ sigma-cli exec), SHGetFolderPath, SHFileOperation
    ← Common file dialogs (GetOpenFileName → Zenith file picker)
```

### sigma-comctl32 (Common Controls)

Provides standard UI widgets (ListView, TreeView, ToolBar, StatusBar, etc.).
Initially stub — return fake HWNDs, implement incrementally.

### Status: `[ ]` Not started

---

## Stage 5 — Registry (`sigma-reg`) and COM (`sigma-com`)

### sigma-reg (Windows Registry)

The Registry is a hierarchical key-value store. Many Win32 apps require it for
settings, COM registration, and file associations.

**Architecture:** sigma-reg maps the registry to a flat SQLite database at
`/sigma/data/registry.db`, queried via sigma-ntdll `NtXxx` key calls.

```
runtime/compat/win32/registry/sigma_reg.cpp
runtime/compat/win32/registry/sigma_reg_hive.cpp    ← HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER
runtime/compat/win32/registry/sigma_reg_persist.cpp ← SQLite backend
include/compat/sigma_reg.h
```

**Key hives:**
```
HKLM\SOFTWARE          → /sigma/data/reg/HKLM/SOFTWARE/
HKCU\Software          → /sigma/data/reg/HKCU/<did>/Software/
HKLM\SYSTEM\CurrentControlSet → hardware config stubs
HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion → version spoofing (build 19041)
```

**sigma-reg → sigma-trustd audit:** all registry writes are ML-DSA-attested,
giving SigmaOS a forensic capability Windows lacks by default.

### sigma-com (COM/OLE/ATL)

Component Object Model is the backbone of Win32 interop — used by Office, IE,
WMI, DirectX, and thousands of enterprise apps.

```
runtime/compat/win32/com/sigma_com.cpp           ← CoInitialize, CoCreateInstance, CoUninitialize
runtime/compat/win32/com/sigma_com_server.cpp    ← IClassFactory, DLL registration
runtime/compat/win32/com/sigma_com_marshal.cpp   ← IStream, IMarshal stubs
runtime/compat/win32/com/sigma_com_automation.cpp ← IDispatch (VBA/scripting)
```

**COM→sigma-bus mapping:**
COM `CoCreateInstance(CLSID)` → sigma-bus `sigma_bus_spawn_service(uuid)`.
COM interfaces become sigma-bus capability tokens. This makes COM objects
first-class SigmaOS services with PQC-attested identity.

### Status: `[ ]` Not started

---

## Stage 6 — D3D/DXVK Bridge + CRT (`sigma-d3d`, `sigma-msvcrt`)

### DXVK Bridge (Direct3D → Vulkan)

Leverages the work already done in `sigma_proton_bridge.cpp` → `mapDxvkSurface()`.
The full DXVK bridge translates Direct3D 9/10/11/12 draw calls to Vulkan.

```
runtime/compat/win32/d3d/sigma_dxvk_bridge.cpp    ← D3D device creation → Vulkan device
runtime/compat/win32/d3d/sigma_dxvk_d3d9.cpp      ← IDirect3DDevice9 → VkCommandBuffer
runtime/compat/win32/d3d/sigma_dxvk_d3d11.cpp     ← ID3D11Device → VkDevice
runtime/compat/win32/d3d/sigma_dxvk_d3d12.cpp     ← D3D12 → Vulkan 1.3 (vkd3d-proton path)
runtime/compat/win32/d3d/sigma_dxgi.cpp            ← IDXGISwapChain → Zenith swapchain
```

**The existing stub to expand:**
```cpp
// Already in sigma_proton_bridge.cpp:
sigma_status mapDxvkSurface(sigma_u32 hwnd, sigma_u32* vulkan_surface) {
    // ← Expand this into full DXVK pipeline
}
```

### sigma-msvcrt (Microsoft C Runtime)

Apps compiled with MSVC link against `msvcrt.dll` / `vcruntime140.dll`.
SigmaOS provides a sovereign reimplementation backed by `klib/sigma_nanolib.h`.

```
runtime/compat/win32/crt/sigma_msvcrt.cpp         ← malloc/free/printf/scanf/etc.
runtime/compat/win32/crt/sigma_msvcrt_math.cpp    ← sin/cos/sqrt (→ libm or AVX-512 inline)
runtime/compat/win32/crt/sigma_msvcrt_string.cpp  ← strcpy/strlen/memcpy (→ sigma_nanolib)
runtime/compat/win32/crt/sigma_msvcrt_io.cpp      ← fopen/fread/fwrite (→ sigma_vfs)
runtime/compat/win32/crt/sigma_msvcrt_threads.cpp ← _beginthread (→ sigma_thread_create)
```

### sigma-winsock2

Network apps use Winsock. Translate to sigma-socket ABI:

```
runtime/compat/win32/winsock/sigma_winsock2.cpp
    ← WSAStartup, socket(), connect(), send(), recv(), closesocket()
    → sigma_socket_open(), sigma_socket_connect(), sigma_socket_send/recv()
```

### Status: `[ ]` Not started

---

## Stage 7 — sigma-wine: Integration Layer

All the above components are assembled into `sigma-wine` — the unified Windows
compatibility environment.

```
runtime/compat/win32/sigma_wine.cpp           ← top-level orchestrator
runtime/compat/win32/sigma_wine_loader.cpp    ← detect PE, invoke sigma-pe, wire DLL stubs
runtime/compat/win32/sigma_wine_server.cpp    ← wineserver equivalent: handles kernel objects
runtime/compat/win32/sigma_wine_debug.cpp     ← PE execution trace, syscall log
include/compat/sigma_wine.h
```

**sigma-wine-server** (equivalent to Wine's `wineserver`):
- Manages cross-process kernel objects (events, mutexes, named pipes, shared memory)
- Runs as a sigma-pod container alongside the Windows app
- Uses sigma-bus for all IPC — no Unix domain sockets required

**CLI interface:**
```bash
sigma-wine notepad.exe
sigma-wine setup.exe /silent
sigma-wine --d3d11 game.exe        # force D3D11→Vulkan
sigma-wine --debug --trace explorer.exe
sigma-wine --prefix /sigma/wine/office office.exe  # isolated Wine prefix
```

### Status: `[ ]` Not started

---
## Phased Timeline

### Phase W0 — Foundation (Months 0–3, parallel to Phase 0 kernel)
*Do this while waiting for kernel boot to stabilize.*

| Task | File | Exit Gate | 
| ------ | ------ | ----------- | 
| PE type definitions header | `include/compat/sigma_pe_types.h` | Compiles cleanly | 
| NT type definitions header | `include/compat/sigma_nt_types.h` | NTSTATUS, UNICODE_STRING, PEB, TEB defined | 
| Handle table skeleton | `runtime/compat/win32/sigma_handle_table.cpp` | alloc/free/lookup working | 
| Registry SQLite schema | `runtime/compat/win32/registry/sigma_reg.cpp` | HKLM/HKCU open/read/write | 
| PE loader (static parser) | `runtime/compat/win32/sigma_pe_loader.cpp` | Validates + dumps sections from a test .exe | 
| sigma-wine CLI stub | `runtime/compat/win32/sigma_wine.cpp` | `sigma-wine --info hello.exe` prints PE headers | 

### Phase W1 — Hello World (Months 3–6)
*Target: run a static Win32 CLI binary that prints to stdout.*

| Task | File | Exit Gate | 
| ------ | ------ | ----------- | 
| NT syscall table (I/O + memory) | `runtime/compat/win32/sigma_ntdll.cpp` | NtReadFile, NtWriteFile, NtAllocateVirtualMemory mapped | 
| PEB/TEB setup | `runtime/compat/win32/sigma_ntdll.cpp` | gs:[0x60] valid, BeingDebugged=0 | 
| sigma-kernel32 console I/O | `runtime/compat/win32/kernel32/sigma_kernel32_console.cpp` | WriteConsoleA writes to stdout | 
| sigma-msvcrt printf | `runtime/compat/win32/crt/sigma_msvcrt.cpp` | printf("Hello") works | 
| sigma-wine exec pipeline | `runtime/compat/win32/sigma_wine_loader.cpp` | Static hello.exe runs end-to-end | 
| QEMU test harness | `tests/compat/win32/test_hello_exe.sh` | CI passes | 

**Milestone:** `sigma-wine hello.exe` prints `Hello, SigmaOS!` in QEMU.

### Phase W2 — CLI App Compatibility (Months 6–12)
*Target: run cmd.exe, Python for Windows, Git for Windows CLI.*

| Task | File | Exit Gate | 
| ------ | ------ | ----------- | 
| Full kernel32 file I/O | `sigma_kernel32_file.cpp` | CreateFile/ReadFile/WriteFile on sigma-vfs | 
| Process creation | `sigma_kernel32_process.cpp` | CreateProcess spawns a child | 
| NT synchronization | `sigma_ntdll.cpp` | Mutex, Event, WaitForSingleObject | 
| Thread creation + TLS | `sigma_kernel32_thread.cpp` | CreateThread + TlsAlloc work | 
| Registry (HKCU read/write) | `sigma_reg.cpp` | App settings persist | 
| sigma-ldso DLL loader | `userland/ldso/sigma_ldso.cpp` | LoadLibrary resolves sigma-kernel32 | 
| sigma-msvcrt complete | `sigma_msvcrt*.cpp` | malloc/free/printf/fopen/fread/fwrite | 
| Winsock2 TCP | `sigma_winsock2.cpp` | socket/connect/send/recv via sigma-net | 

**Milestone:** `sigma-wine python.exe -c "print('hello')"` works.

### Phase W3 — GUI App Compatibility (Months 12–24)
*Target: run Notepad, simple Win32 GUI apps.*

| Task | File | Exit Gate | 
| ------ | ------ | ----------- | 
| sigma-user32 window + WM_PAINT | `sigma_user32_window.cpp` | CreateWindow/ShowWindow/GetMessage loop | 
| HWND → Zenith surface mapping | `sigma_user32_dc.cpp` | Window appears on Zenith desktop | 
| Message pump | `sigma_user32_msg.cpp` | WM_KEYDOWN/WM_MOUSEMOVE delivered | 
| sigma-gdi32 text + basic draw | `sigma_gdi32_draw.cpp` | TextOut, Rectangle, FillRect via Vulkan | 
| GDI font (HarfBuzz bridge) | `sigma_gdi32_font.cpp` | CreateFont renders in Zenith | 
| sigma-comctl32 stubs | `sigma_comctl32.cpp` | ListView/TreeView return valid HWNDs | 
| sigma-shell32 stubs | `sigma_shell32.cpp` | ShellExecute, GetOpenFileName | 
| sigma-wine-server IPC | `sigma_wine_server.cpp` | Cross-process handles work | 

**Milestone:** `sigma-wine notepad.exe` opens, renders text, saves a file.

### Phase W4 — Office / Enterprise Apps (Months 24–36)
*Target: run Office 2019 / LibreOffice Windows build, VSCode, Electron apps.*

| Task | File | Exit Gate | 
| ------ | ------ | ----------- | 
| COM/OLE core | `sigma_com.cpp` | CoCreateInstance works for registered CLSIDs | 
| IDispatch (VBA/scripting) | `sigma_com_automation.cpp` | VBA macros execute | 
| sigma-gdi32 BitBlt/DIBits | `sigma_gdi32_bitmap.cpp` | Image rendering works | 
| sigma-d3d11 via DXVK | `sigma_dxvk_d3d11.cpp` | D3D11 device creates on Vulkan | 
| WIC (Windows Imaging) stubs | `sigma_windowscodecs.cpp` | PNG/JPEG decode via libpng/libjpeg | 
| sigma-rpc (MSRPC stubs) | `sigma_rpc.cpp` | OLE automation over sigma-bus | 
| .NET CLR bootstrap | `runtime/compat/win32/dotnet/sigma_clr_bootstrap.cpp` | .NET 6+ apps reach Main() | 
| Electron/Node.js bridge | `runtime/compat/win32/sigma_node_bridge.cpp` | Chromium renderer starts | 

**Milestone:** Microsoft Word 2019 opens and renders a document.

### Phase W5 — Gaming (Months 24–48, parallel to W4)
*Target: run Steam + Proton games natively on SigmaOS.*

| Task | File | Exit Gate | 
| ------ | ------ | ----------- | 
| DXVK D3D9/D3D11 complete | `sigma_dxvk_d3d9.cpp`, `sigma_dxvk_d3d11.cpp` | Games render frames | 
| vkd3d-proton D3D12 | `sigma_dxvk_d3d12.cpp` | DX12 titles run | 
| XInput (gamepad) | `runtime/compat/win32/sigma_xinput.cpp` | Controller input works | 
| XAudio2 → sigma-audio | `runtime/compat/win32/sigma_xaudio2.cpp` | Game audio plays | 
| Steam client compat | integration test | Steam launches, downloads a game | 
| sigma-gamescope | `zenith_desktop/gamescope/sigma_gamescope.cpp` | Dedicated gaming compositor mode | 

**Milestone:** A DX11 Steam game runs at playable framerate on SigmaOS.

---

## Complete File Tree

```
runtime/compat/win32/
├── sigma_wine.cpp                     Stage 7 — top-level orchestrator
├── sigma_wine_loader.cpp              Stage 7 — PE detection + DLL wiring
├── sigma_wine_server.cpp              Stage 7 — wineserver equivalent
├── sigma_wine_debug.cpp               Stage 7 — trace/logging
├── sigma_pe_loader.cpp                Stage 1 — PE32+ parser
├── sigma_pe_loader.h
├── sigma_handle_table.cpp             Stage 2 — HANDLE → sigma object
├── sigma_handle_table.h
├── sigma_ntdll.cpp                    Stage 2 — NT native API
├── sigma_nt_syscall_table.cpp         Stage 2 — NT→sigma number map
├── kernel32/
│   ├── sigma_kernel32.cpp             Stage 3
│   ├── sigma_kernel32_file.cpp
│   ├── sigma_kernel32_process.cpp
│   ├── sigma_kernel32_memory.cpp
│   ├── sigma_kernel32_sync.cpp
│   ├── sigma_kernel32_thread.cpp
│   ├── sigma_kernel32_time.cpp
│   ├── sigma_kernel32_console.cpp
│   ├── sigma_kernel32_module.cpp
│   └── sigma_kernel32_error.cpp
├── user32/
│   ├── sigma_user32.cpp               Stage 3
│   ├── sigma_user32_window.cpp
│   ├── sigma_user32_msg.cpp
│   ├── sigma_user32_paint.cpp
│   ├── sigma_user32_input.cpp
│   ├── sigma_user32_dialog.cpp
│   └── sigma_user32_dc.cpp
├── gdi32/
│   ├── sigma_gdi32.cpp                Stage 4
│   ├── sigma_gdi32_draw.cpp
│   ├── sigma_gdi32_font.cpp
│   ├── sigma_gdi32_bitmap.cpp
│   └── sigma_gdi32_dc.cpp
├── shell32/
│   └── sigma_shell32.cpp              Stage 4
├── comctl32/
│   └── sigma_comctl32.cpp             Stage 4
├── registry/
│   ├── sigma_reg.cpp                  Stage 5
│   ├── sigma_reg_hive.cpp
│   └── sigma_reg_persist.cpp
├── com/
│   ├── sigma_com.cpp                  Stage 5
│   ├── sigma_com_server.cpp
│   ├── sigma_com_marshal.cpp
│   └── sigma_com_automation.cpp
├── d3d/
│   ├── sigma_dxvk_bridge.cpp          Stage 6
│   ├── sigma_dxvk_d3d9.cpp
│   ├── sigma_dxvk_d3d11.cpp
│   ├── sigma_dxvk_d3d12.cpp
│   └── sigma_dxgi.cpp
├── crt/
│   ├── sigma_msvcrt.cpp               Stage 6
│   ├── sigma_msvcrt_math.cpp
│   ├── sigma_msvcrt_string.cpp
│   ├── sigma_msvcrt_io.cpp
│   └── sigma_msvcrt_threads.cpp
├── winsock/
│   └── sigma_winsock2.cpp             Stage 6
├── dotnet/
│   └── sigma_clr_bootstrap.cpp        Stage W4
└── sigma_xinput.cpp                   Stage W5

include/compat/
├── sigma_pe_types.h                   PE32+ structures
├── sigma_nt_types.h                   NTSTATUS, PEB, TEB, UNICODE_STRING
├── sigma_nt_syscalls.h                NtXxx declarations
├── sigma_win32_types.h                HWND, HDC, HANDLE, WNDCLASS, MSG
├── sigma_reg.h                        Registry API
└── sigma_wine.h                       sigma-wine public API
```


## Compatibility Targets by Phase

| Application | Category | Phase | Key blockers | 
| ------------- | ---------- | ------- | -------------- | 
| `hello.exe` (static Win32 CLI) | CLI | W1 | PE loader + kernel32 console I/O | 
| Python 3.x for Windows | CLI | W2 | CRT + kernel32 + process creation | 
| Git for Windows | CLI | W2 | CRT + file I/O + winsock | 
| 7-Zip CLI | CLI | W2 | CRT + file I/O | 
| Notepad | GUI | W3 | user32 + GDI text + message pump | 
| Paint | GUI | W3 | GDI bitmap + user32 | 
| PuTTY | Network GUI | W3 | Winsock + user32 | 
| VSCode | Electron GUI | W4 | Electron/Node + D3D11 (Chromium GPU) | 
| Microsoft Word 2019 | Office | W4 | COM + GDI + RPC | 
| Microsoft Excel 2019 | Office | W4 | COM + VBA + GDI | 
| LibreOffice (Win32 build) | Office | W4 | CRT + GDI + COM | 
| .NET 6+ console apps | .NET | W4 | CLR bootstrap | 
| DirectX 9 games | Gaming | W5 | DXVK D3D9 + sigma-audio | 
| DirectX 11 games | Gaming | W5 | DXVK D3D11 + XInput | 
| DirectX 12 games | Gaming | W5 | vkd3d-proton + DX12 caps | 
| Steam client | Gaming platform | W5 | Chromium + D3D11 + Winsock | 

---

## Security Model for the Compat Layer

The compat layer runs **entirely in userspace** — no NT kernel code runs in SigmaOS Ring-0.
This is a fundamental security advantage over running Windows inside a VM.

```
Windows app (Ring-3)
    │  NT syscall (SYSCALL instruction)
    ▼
sigma-ntdll syscall gate (Ring-3, intercepted before kernel entry)
    │  translates NtXxx → sigma-syscall
    ▼
SigmaOS kernel (Ring-0, sigma-syscall ABI)
    │
    ▼
SigmaOS hardware
```

**Sandboxing:** every `sigma-wine` process runs inside a sigma-mac capability sandbox.
The Windows app cannot access sigma-trustd keys, sigma-pod namespaces, or India Stack
APIs unless explicitly granted in the `.sigma-policy` file.

**Registry isolation:** each `sigma-wine` prefix has its own registry database.
A malicious app that corrupts `HKLM\SOFTWARE` cannot affect other prefixes or the
SigmaOS native environment.

**PQC-attested DLLs:** all sigma-wine DLL stubs (sigma-kernel32, sigma-ntdll, etc.)
are ML-DSA-signed. A compromised DLL cannot be substituted without breaking the
Dilithium attestation chain — something Wine has no equivalent of.

**Audit trail:** every NT syscall translation is logged to sigma-audit with a
DID-signed timestamp. Full replay-proof audit of what a Windows app did — impossible
on real Windows without expensive third-party tooling.

---

## SigmaOS Advantages Over Running Windows in a VM

| Dimension | Windows in VM (Hyper-V/KVM) | sigma-wine (native compat) | 
| ----------- | ---------------------------- | --------------------------- | 
| RAM overhead | +2–4 GB for Windows guest OS | Zero — no guest OS | 
| Boot time | 30–60 seconds for VM | Instant — just process launch | 
| GPU passthrough | Complex, limited support | Native Vulkan via DXVK | 
| File access | Shared folders (slow) | Direct sigma-vfs access | 
| Security isolation | Hypervisor boundary | sigma-mac + capability sandbox | 
| Audit trail | None (black box) | Full NT syscall audit log | 
| PQC protection | None | ML-DSA-signed DLL stubs | 
| Integration | Clipboard, network bridges | Full sigma-bus IPC integration | 
| India Stack | Not available in guest | sigma-sdk available to Win32 apps | 

---

## Test Plan

### Unit tests (per stage)
```
tests/compat/win32/
├── test_pe_loader.cpp         ← parse known PE files, verify section map
├── test_nt_syscall_table.cpp  ← every mapped NT call returns expected value
├── test_handle_table.cpp      ← alloc/free/lookup 10,000 handles no leak
├── test_registry.cpp          ← HKCU read/write/delete round-trip
├── test_peb_teb.cpp           ← PEB fields correct, gs:[0x60] readable
├── test_kernel32_io.cpp       ← CreateFile/ReadFile/WriteFile on sigma-vfs
├── test_kernel32_process.cpp  ← CreateProcess + WaitForSingleObject
├── test_msvcrt.cpp            ← printf, malloc, fopen, fread, fwrite
└── test_user32_window.cpp     ← CreateWindow → Zenith surface appears
```

### Integration tests (per milestone)
```
tests/compat/win32/integration/
├── run_hello_exe.sh           ← W1: sigma-wine static hello.exe → "Hello, SigmaOS!"
├── run_python_cli.sh          ← W2: sigma-wine python.exe -c "print('ok')"
├── run_git_clone.sh           ← W2: sigma-wine git.exe clone <repo>
├── run_notepad.sh             ← W3: sigma-wine notepad.exe opens + renders
├── run_vscode.sh              ← W4: sigma-wine code.exe starts
└── run_dx11_triangle.sh       ← W5: D3D11 triangle demo renders via DXVK
```

### CI gating
```yaml
# .github/workflows/sigma_wine_ci.yml
name: sigma-wine compatibility CI
on: [push, pull_request]
jobs:
  wine-unit:
    runs-on: ubuntu-latest
    steps:
      - run: make test-compat-win32
  wine-pe-loader:
    steps:
      - run: ./tests/compat/win32/run_hello_exe.sh
```

---

## Master Status Checklist

| Stage | Component | Status | Phase | 
| ------- | ----------- | -------- | ------- | 
| W0 | `include/compat/sigma_pe_types.h` | `[ ]` | Now | 
| W0 | `include/compat/sigma_nt_types.h` | `[ ]` | Now | 
| W0 | `include/compat/sigma_win32_types.h` | `[ ]` | Now | 
| W0 | `include/compat/sigma_wine.h` | `[ ]` | Now | 
| W0 | `sigma_handle_table.cpp` (skeleton) | `[ ]` | Now | 
| W0 | `sigma_reg.cpp` (SQLite schema) | `[ ]` | Now | 
| W0 | `sigma_pe_loader.cpp` (static parser) | `[ ]` | Now | 
| W1 | `sigma_ntdll.cpp` (I/O + memory) | `[ ]` | Month 3 | 
| W1 | `sigma_kernel32_console.cpp` | `[ ]` | Month 3 | 
| W1 | `sigma_msvcrt.cpp` (printf subset) | `[ ]` | Month 3 | 
| W1 | `sigma_wine_loader.cpp` | `[ ]` | Month 3 | 
| W2 | `sigma_kernel32_file.cpp` | `[ ]` | Month 6 | 
| W2 | `sigma_kernel32_process.cpp` | `[ ]` | Month 6 | 
| W2 | `sigma_kernel32_thread.cpp` | `[ ]` | Month 6 | 
| W2 | `sigma_winsock2.cpp` | `[ ]` | Month 6 | 
| W2 | `sigma_ldso.cpp` (DLL loading) | `[ ]` | Month 6 | 
| W3 | `sigma_user32_window.cpp` | `[ ]` | Month 12 | 
| W3 | `sigma_user32_msg.cpp` | `[ ]` | Month 12 | 
| W3 | `sigma_gdi32_draw.cpp` | `[ ]` | Month 18 | 
| W3 | `sigma_wine_server.cpp` | `[ ]` | Month 18 | 
| W4 | `sigma_com.cpp` | `[ ]` | Month 24 | 
| W4 | `sigma_dxvk_d3d11.cpp` | `[ ]` | Month 24 | 
| W4 | `sigma_clr_bootstrap.cpp` | `[ ]` | Month 30 | 
| W5 | `sigma_dxvk_d3d9.cpp` | `[ ]` | Month 24 | 
| W5 | `sigma_dxvk_d3d12.cpp` | `[ ]` | Month 36 | 
| W5 | `sigma_xinput.cpp` | `[ ]` | Month 30 | 
| W5 | `sigma_gamescope.cpp` | `[ ]` | Month 36 | 

---

*See also: [Windows Parity Roadmap](Windows-Parity-Roadmap) · [Gap Analysis](Gap-Analysis) · [Development Roadmap](Development-Roadmap) · [System Improvement Plan](System-Improvement-Plan)*
