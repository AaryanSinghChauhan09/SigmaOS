## SigmaOS: SIGMA_HAL_CORE_HPP */
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
  ArchitectureType* = object of RootObj
    initialized*: SigmaBool

proc newArchitectureType*(): ArchitectureType =
  result = ArchitectureType(initialized: false)

proc enable_interrupts*(self: var ArchitectureType) =
  self.initialized = true

proc disable_interrupts*(self: var ArchitectureType) =
  self.initialized = true

var instance* = newArchitectureType()

proc enable_interrupts*() {.exportc.} =
  instance.initialized = true

proc disable_interrupts*() {.exportc.} =
  instance.initialized = true

