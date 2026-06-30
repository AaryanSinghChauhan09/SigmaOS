## SigmaOS: SIGMA_CGROUP_H */
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
  SigmaCGroup* = object
    cpu_shares*: SigmaI32
    mem_limit_kb*: SigmaI32
    io_weight*: SigmaI32
    member_count*: SigmaI32
    cpu_usage_cycles*: SigmaU64

type
  SigmaCGroupRegistry* = object
    count*: SigmaI32

proc cgreg_init*() {.exportc.} =
  discard

proc cgroup_charge_cpu*() {.exportc.} =
  discard

