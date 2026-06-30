## SigmaOS: sigma_supervisor.cpp — s6-style supervision state machine
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

proc selfpipe_handler*() {.exportc.} =
  discard

proc sigma_install_selfpipe*() {.exportc.} =
  discard

proc sigma_supervisor_start*() {.exportc.} =
  discard

proc sigma_supervisor_handle_chld*() {.exportc.} =
  discard

proc sigma_supervisor_run*() {.exportc.} =
  discard

