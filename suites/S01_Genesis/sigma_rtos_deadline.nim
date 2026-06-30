## SigmaOS: SIGMA_RTOS_DEADLINE_HPP */
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
  EDFScheduler* = object of RootObj
    initialized*: SigmaBool

proc newEDFScheduler*(): EDFScheduler =
  result = EDFScheduler(initialized: false)

proc register_task*(self: var EDFScheduler) =
  self.initialized = true

type
  DeadlineTask* = object
    absolute_deadline_rdtsc*: SigmaU64
    worst_case_execution_time*: SigmaU64
    is_hard_realtime*: SigmaBool

var instance* = newEDFScheduler()

