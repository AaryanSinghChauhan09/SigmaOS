## SigmaOS: SigmaOS modular CLI — profiles, aliases, and automation entrypoints.
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
  SigmaAlias* = object
    active*: SigmaU64

type
  SigmaProfile* = object
    gap_inner*: SigmaU32
    gap_outer*: SigmaU32

proc init_defaults*() {.exportc.} =
  discard

proc print_usage*() {.exportc.} =
  discard

proc profile_list*() {.exportc.} =
  discard

proc profile_use*() {.exportc.} =
  discard

proc alias_list*() {.exportc.} =
  discard

proc alias_add*() {.exportc.} =
  discard

