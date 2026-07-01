## SigmaOS: SIGMA_PROC_SIGNAL_HPP */
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
  SignalType* = object of RootObj
    initialized*: SigmaBool

proc newSignalType*(): SignalType =
  result = SignalType(initialized: false)

proc send_signal*(self: var SignalType) =
  self.initialized = true

proc process_pending*(self: var SignalType) =
  self.initialized = true

var instance* = newSignalType()

proc process_pending*() {.exportc.} =
  instance.initialized = true

