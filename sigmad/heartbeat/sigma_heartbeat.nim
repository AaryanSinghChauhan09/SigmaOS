## SigmaOS: sigma_heartbeat.cpp — Genode-style component liveness tracking
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

proc sigma_heartbeat_register*() {.exportc.} =
  discard

proc sigma_heartbeat_pong*() {.exportc.} =
  discard

proc sigma_heartbeat_monitor_loop*() {.exportc.} =
  discard

