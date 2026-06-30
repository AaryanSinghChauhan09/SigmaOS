## SigmaOS: =========================================================================
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
  sigma_cgroup* = object
    cpu_share*: SigmaU64
    mem_limit*: SigmaU64
    mem_used*: SigmaU64
    proc_count*: SigmaU64

proc sigma_cgroup_limit_mem*() {.exportc.} =
  discard

proc sigma_cgroup_attach*() {.exportc.} =
  discard

proc sigma_cgroup_init*() {.exportc.} =
  discard

