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
  ZenithApp* = object
    state*: SigmaU64
    theme_override*: SigmaU32
    active*: SigmaU64

proc app_manager_init*() {.exportc.} =
  discard

proc app_switch_state*() {.exportc.} =
  discard

proc app_personalize*() {.exportc.} =
  discard

