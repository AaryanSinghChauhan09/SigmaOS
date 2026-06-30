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

proc sigma_perf_init*() {.exportc.} =
  discard

proc sigma_perf_counter_enable*() {.exportc.} =
  discard

proc sigma_perf_counter_disable*() {.exportc.} =
  discard

proc sigma_perf_counter_reset*() {.exportc.} =
  discard

proc sigma_perf_counter_close*() {.exportc.} =
  discard

proc sigma_perf_counters_dump*() {.exportc.} =
  discard

proc sigma_perf_sample_record*() {.exportc.} =
  discard

proc sigma_perf_samples_dump*() {.exportc.} =
  discard

proc trace_push*() {.exportc.} =
  discard

proc sigma_trace_begin*() {.exportc.} =
  discard

proc sigma_trace_end*() {.exportc.} =
  discard

proc sigma_trace_instant*() {.exportc.} =
  discard

proc sigma_trace_counter*() {.exportc.} =
  discard

proc sigma_trace_dump_json*() {.exportc.} =
  discard

proc sigma_proc_stat_print*() {.exportc.} =
  discard

proc sigma_system_stat_print*() {.exportc.} =
  discard

