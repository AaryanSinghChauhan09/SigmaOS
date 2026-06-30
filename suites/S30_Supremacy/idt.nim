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
  SigmaInterruptFrame* = object
    vector*: SigmaU64
    error_code*: SigmaU64

proc pic_remap*() {.exportc.} =
  discard

proc pic_eoi*() {.exportc.} =
  discard

proc pic_unmask_irq*() {.exportc.} =
  discard

proc idt_set_gate*() {.exportc.} =
  discard

proc idt_init*() {.exportc.} =
  discard

proc idt_register_handler*() {.exportc.} =
  discard

proc sigma_interrupt_handler*() {.exportc.} =
  discard

