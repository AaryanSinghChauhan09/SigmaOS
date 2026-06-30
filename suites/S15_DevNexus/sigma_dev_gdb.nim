## SigmaOS: SIGMA_DEV_GDB_H */
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
  SigmaDebugBreakpoint* = object
    addr*: SigmaU64
    original_byte*: uint8
    state*: SigmaU64
    hit_count*: SigmaU64

type
  SigmaDebugStub* = object
    bpt_count*: SigmaI32
    attached*: uint8
    last_trap_addr*: SigmaU64

proc dbg_init*() {.exportc.} =
  discard

proc dbg_on_trap*() {.exportc.} =
  discard

