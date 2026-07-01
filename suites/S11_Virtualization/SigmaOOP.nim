## SigmaOS: SigmaOOP.hpp — Sovereign canonical shim */
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
  SigmaMemory* = object of RootObj
    initialized*: SigmaBool

proc newSigmaMemory*(): SigmaMemory =
  result = SigmaMemory(initialized: false)

proc sigma_log*(self: var SigmaMemory) =
  self.initialized = true

var instance* = newSigmaMemory()

proc sigma_log*() {.exportc.} =
  instance.initialized = true

