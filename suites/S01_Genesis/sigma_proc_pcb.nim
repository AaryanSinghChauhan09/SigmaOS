## SigmaOS: SIGMA_PROC_PCB_H */
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
  SigmaPCB* = object
    pid*: SigmaU64
    cr3*: SigmaU64
    rsp*: SigmaU64
    rip*: SigmaU64
    state*: SigmaI32
    priority*: SigmaI32
    cpu_cycles*: SigmaU64

type
  SigmaPCBTable* = object
    count*: SigmaI32
    next_pid*: SigmaI32

proc pcb_table_init*() {.exportc.} =
  discard

proc pcb_free*() {.exportc.} =
  discard

