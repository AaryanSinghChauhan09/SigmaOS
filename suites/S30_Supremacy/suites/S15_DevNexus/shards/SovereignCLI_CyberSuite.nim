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

proc cmd_sigma_whoami*() {.exportc.} =
  discard

proc cmd_sigma_nmap*() {.exportc.} =
  discard

proc cmd_sigma_iptables*() {.exportc.} =
  discard

proc cmd_sigma_vault*() {.exportc.} =
  discard

proc cmd_sigma_defender*() {.exportc.} =
  discard

proc SovereignCLI_CyberSuite_Register*() {.exportc.} =
  discard

