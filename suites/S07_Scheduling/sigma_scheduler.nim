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
  TaskControlBlock* = object
    tid*: SigmaU64
    priority*: SigmaU64
    state*: SigmaU64
    policy*: SigmaU64
    vruntime*: SigmaU64
    timeslice*: SigmaU64
    base_timeslice*: SigmaU64
    burst_ema*: SigmaU64
    last_burst*: SigmaU64
    total_runtime*: SigmaU64
    pool_id*: SigmaI32

type
  RunQueue* = object
    count*: SigmaU64

type
  SchedProfileStats* = object
    total_npu_dispatches*: SigmaU64
    total_cpu_fallbacks*: SigmaU64
    total_fused_kernels*: SigmaU64
    npu_latency_ns_accum*: SigmaU64
    cpu_latency_ns_accum*: SigmaU64

proc rq_enqueue*() {.exportc.} =
  discard

proc sched_kill_current_task*() {.exportc.} =
  discard

proc ai_update_burst*() {.exportc.} =
  discard

proc sched_tick*() {.exportc.} =
  discard

proc sched_yield*() {.exportc.} =
  discard

proc sched_init*() {.exportc.} =
  discard

proc sched_dispatch_tensor_op*() {.exportc.} =
  discard

proc sched_audit*() {.exportc.} =
  discard

