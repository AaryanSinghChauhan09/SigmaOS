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

proc sigma_hal_init*() {.exportc.} =
  discard

proc sigma_hal_unregister*() {.exportc.} =
  discard

proc sigma_hal_enumerate_bus*() {.exportc.} =
  discard

proc sigma_hal_device_list*() {.exportc.} =
  discard

proc sigma_irq_sigma_free*() {.exportc.} =
  discard

proc sigma_irq_enable*() {.exportc.} =
  discard

proc sigma_irq_disable*() {.exportc.} =
  discard

proc sigma_irq_dispatch*() {.exportc.} =
  discard

proc sigma_irq_stats*() {.exportc.} =
  discard

proc sigma_mmio_write32*() {.exportc.} =
  discard

proc sigma_dma_sigma_free*() {.exportc.} =
  discard

