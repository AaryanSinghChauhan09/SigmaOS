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
  KeyboardShortcut* = object
    modifier*: SigmaU32
    key_code*: SigmaU32
    target_shard*: SigmaU32
    active*: SigmaU64

proc keyboard_master_init*() {.exportc.} =
  discard

proc keyboard_on_event*() {.exportc.} =
  discard

