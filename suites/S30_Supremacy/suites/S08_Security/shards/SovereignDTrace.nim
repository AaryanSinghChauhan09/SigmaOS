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

proc sigma_dt_probe_fire*() {.exportc.} =
  discard

proc sigma_dt_list_probes*() {.exportc.} =
  discard

proc sigma_dt_aggr_print*() {.exportc.} =
  discard

proc sigma_pf_show_rules*() {.exportc.} =
  discard

proc sigma_pf_show_states*() {.exportc.} =
  discard

proc sigma_pf_show_info*() {.exportc.} =
  discard

proc trace_action*() {.exportc.} =
  discard

proc SovereignDTrace_Init*() {.exportc.} =
  discard

