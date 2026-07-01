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
  SigmaAgent* = object
    plan_steps*: SigmaU64
    current_step*: SigmaU64

proc agent_init*() {.exportc.} =
  discard

proc agent_execute_step*() {.exportc.} =
  discard

proc agent_dispatch_mission*() {.exportc.} =
  discard

