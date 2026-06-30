## SigmaOS: Σ SIGMAOS: SOVEREIGN INTERRUPT DISPATCHER
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
  idt_entry* = object
    base_low*: SigmaU16
    selector*: SigmaU16
    ist*: SigmaU8
    flags*: SigmaU8
    base_mid*: SigmaU16
    base_high*: SigmaU32
    reserved*: SigmaU32

type
  idtr* = object
    limit*: SigmaU16
    base*: SigmaU64

proc sigma_set_idt_gate*() {.exportc.} =
  discard

proc sigma_interrupt_handler*() {.exportc.} =
  discard

proc sigma_idt_init*() {.exportc.} =
  discard

