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

type
  sigma_rcu_head* = object

type
  sigma_rcu_node* = object
    active_readers*: SigmaU64
    generation*: SigmaU64

proc sigma_rcu_read_lock*() {.exportc.} =
  discard

proc sigma_rcu_read_unlock*() {.exportc.} =
  discard

proc sigma_synchronize_rcu*() {.exportc.} =
  discard

proc sigma_rcu_init*() {.exportc.} =
  discard

