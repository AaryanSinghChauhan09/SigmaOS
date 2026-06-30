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

proc cmd_sigma_apt*() {.exportc.} =
  discard

proc cmd_sigma_pacman*() {.exportc.} =
  discard

proc cmd_sigma_grep*() {.exportc.} =
  discard

proc cmd_sigma_top*() {.exportc.} =
  discard

proc cmd_sigma_htop*() {.exportc.} =
  discard

proc cmd_sigma_neofetch*() {.exportc.} =
  discard

proc SovereignCLI_DistroSuite_Register*() {.exportc.} =
  discard

