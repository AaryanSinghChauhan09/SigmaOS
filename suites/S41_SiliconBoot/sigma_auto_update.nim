## SigmaOS: SIGMA_AUTO_UPDATE_HPP */
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
  SeamlessUpdater* = object of RootObj
    initialized*: SigmaBool

proc newSeamlessUpdater*(): SeamlessUpdater =
  result = SeamlessUpdater(initialized: false)

proc stage_update*(self: var SeamlessUpdater) =
  self.initialized = true

proc commit_and_reboot*(self: var SeamlessUpdater) =
  self.initialized = true

type
  UpdatePayload* = object
    size*: SigmaI32
    fnv1a_checksum*: SigmaU64

var instance* = newSeamlessUpdater()

proc commit_and_reboot*() {.exportc.} =
  instance.initialized = true

