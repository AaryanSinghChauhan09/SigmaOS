## SigmaOS: SigmaCron module
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
  SigmaCron* = object of RootObj
    initialized*: SigmaBool

proc newSigmaCron*(): SigmaCron =
  result = SigmaCron(initialized: false)

proc add_job*(self: var SigmaCron) =
  self.initialized = true

proc tick*(self: var SigmaCron) =
  self.initialized = true

type
  CronJob* = object
    job_id*: SigmaU64
    active*: SigmaBool

var instance* = newSigmaCron()

proc add_job*() {.exportc.} =
  instance.initialized = true

proc tick*() {.exportc.} =
  instance.initialized = true

