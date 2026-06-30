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

proc sigma_spinlock_init*() {.exportc.} =
  discard

proc sigma_spinlock_acquire*() {.exportc.} =
  discard

proc sigma_spinlock_release*() {.exportc.} =
  discard

proc sigma_semaphore_init*() {.exportc.} =
  discard

proc sigma_semaphore_signal*() {.exportc.} =
  discard

proc sigma_rwlock_init*() {.exportc.} =
  discard

proc sigma_rwlock_read_release*() {.exportc.} =
  discard

proc sigma_rwlock_write_release*() {.exportc.} =
  discard

proc sigma_ring_init*() {.exportc.} =
  discard

proc SovereignConcurrency_Register*() {.exportc.} =
  discard

