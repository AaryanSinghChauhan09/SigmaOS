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

proc cpu_halt*() {.exportc.} =
  discard

proc cpu_pause*() {.exportc.} =
  discard

proc cpu_fence*() {.exportc.} =
  discard

proc cpu_sti*() {.exportc.} =
  discard

proc cpu_cli*() {.exportc.} =
  discard

proc cpu_write_cr3*() {.exportc.} =
  discard

proc cpu_invlpg*() {.exportc.} =
  discard

proc port_outb*() {.exportc.} =
  discard

proc port_outw*() {.exportc.} =
  discard

proc sigma_memcpy*() {.exportc.} =
  discard

proc sigma_memset*() {.exportc.} =
  discard

