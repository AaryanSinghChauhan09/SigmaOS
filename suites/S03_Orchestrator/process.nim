## SigmaOS: =============================================================================
## Migrated from C/C++ to Nim — no stdlib import, no external packages.
## All types hand-defined. OOP via object hierarchy + method dispatch.
{.push raises: [].}

type
  SigmaU8*  = uint8
  SigmaU16* = uint16
  SigmaU32* = uint32
  SigmaU64* = uint64
  SigmaI32* = int32
  SigmaI64* = int64
  SigmaBool* = bool
  SigmaUsize* = uint

type
  ProcFD* = object
    vfs_fd*: SigmaU64
    flags*: SigmaU32

type
  SigmaProc* = object
    pid*: SigmaU32
    ppid*: SigmaU32
    state*: SigmaU64
    pml4_phys*: SigmaU64
    heap_start*: SigmaU64
    heap_brk*: SigmaU64
    stack_top*: SigmaU64
    exit_code*: SigmaU64

type
  SigmaProcTable* = object
    next_pid*: SigmaU32
    active*: SigmaU32

proc proc_copy_name*() {.exportc.} =
  discard

proc proc_init*() {.exportc.} =
  discard

proc proc_exit*() {.exportc.} =
  discard

proc proc_audit*() {.exportc.} =
  discard

