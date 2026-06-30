## SigmaOS: sigma_gov module
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

proc sigma_gov_rti_draft*() {.exportc.} =
  discard

proc sigma_gov_gem_order*() {.exportc.} =
  discard

proc sigma_gov_roster_check*() {.exportc.} =
  discard

proc sigma_gov_gfr_procurement*() {.exportc.} =
  discard

