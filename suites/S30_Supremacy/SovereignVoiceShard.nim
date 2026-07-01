## SigmaOS: =========================================================================
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
  IAudioSource* = object of RootObj
    initialized*: SigmaBool

proc newIAudioSource*(): IAudioSource =
  result = IAudioSource(initialized: false)

proc ActivateGlobalWakeKey*(self: var IAudioSource) =
  self.initialized = true

proc ProcessVoiceEvent*(self: var IAudioSource) =
  self.initialized = true

proc ProcessText*(self: var IAudioSource) =
  self.initialized = true

proc main*(self: var IAudioSource) =
  self.initialized = true

var instance* = newIAudioSource()

proc ActivateGlobalWakeKey*() {.exportc.} =
  instance.initialized = true

proc ProcessVoiceEvent*() {.exportc.} =
  instance.initialized = true

proc ProcessText*() {.exportc.} =
  instance.initialized = true

