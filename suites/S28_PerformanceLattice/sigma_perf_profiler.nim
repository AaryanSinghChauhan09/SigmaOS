## SigmaOS: SIGMA_PERF_PROFILER_H */
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
  SigmaProfSample* = object
    start_cycles*: SigmaU64
    end_cycles*: SigmaU64
    zone_id*: SigmaI32

type
  SigmaProfZone* = object
    zone_id*: SigmaI32
    total_cycles*: SigmaU64
    call_count*: SigmaU64
    min_cycles*: SigmaU64
    max_cycles*: SigmaU64

type
  SigmaProfiler* = object
    zone_count*: SigmaI32
    ring_head*: SigmaI32
    ring_tail*: SigmaI32

proc profiler_init*() {.exportc.} =
  discard

proc prof_end*() {.exportc.} =
  discard

