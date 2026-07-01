## SigmaOS: SIGMA_NUMA_H */
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
  SigmaNumaNode* = object
    slab*: SigmaU64
    lock*: SigmaU64
    node_id*: SigmaI32
    total_alloc_bytes*: SigmaU64
    total_free_bytes*: SigmaU64

type
  SigmaNumaAllocator* = object
    node_count*: SigmaI32

proc numa_init*() {.exportc.} =
  discard

proc numa_free_on*() {.exportc.} =
  discard

