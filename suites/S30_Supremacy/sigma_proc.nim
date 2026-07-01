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
  SovereignPCB* = object
    pid*: SigmaU64
    cr3*: SigmaU64
    rsp*: SigmaU64
    state*: SigmaU32

type
  SovereignProcessManager* = object
    active_count*: SigmaU32
    kills*: SigmaU32

proc tlb_flush*() {.exportc.} =
  discard

proc ctx_switch_shard*() {.exportc.} =
  discard

proc sigma_proc_audit*() {.exportc.} =
  discard

proc sigma_proc_init*() {.exportc.} =
  discard

