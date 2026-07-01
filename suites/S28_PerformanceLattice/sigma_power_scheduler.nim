## SigmaOS: SIGMA_POWER_SCHEDULER_HPP */
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
  PowerAwareScheduler* = object of RootObj
    initialized*: SigmaBool

proc newPowerAwareScheduler*(): PowerAwareScheduler =
  result = PowerAwareScheduler(initialized: false)

proc throttle_background_tasks*(self: var PowerAwareScheduler) =
  self.initialized = true

var instance* = newPowerAwareScheduler()

proc throttle_background_tasks*() {.exportc.} =
  instance.initialized = true

