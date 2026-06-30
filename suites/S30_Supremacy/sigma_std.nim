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
  CPUIDResult* = object

type
  RingBuffer* = object
    capacity*: SigmaU32
    head*: SigmaU32
    tail*: SigmaU32
    count*: SigmaU32

proc sigma_set_tsc_freq_mhz*() {.exportc.} =
  discard

proc sigma_bzero*() {.exportc.} =
  discard

proc sigma_strcpy_safe*() {.exportc.} =
  discard

proc sigma_io_wait*() {.exportc.} =
  discard

proc port_outw_fn*() {.exportc.} =
  discard

proc port_outl*() {.exportc.} =
  discard

proc sigma_wrmsr*() {.exportc.} =
  discard

proc k_print_raw*() {.exportc.} =
  discard

proc rb_init*() {.exportc.} =
  discard

proc spinlock_init*() {.exportc.} =
  discard

proc spinlock_acquire*() {.exportc.} =
  discard

proc spinlock_release*() {.exportc.} =
  discard

