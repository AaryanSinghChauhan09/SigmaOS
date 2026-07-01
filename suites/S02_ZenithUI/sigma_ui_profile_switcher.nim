## SigmaOS: SIGMA_UI_PROFILE_SWITCHER_H */
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
  UIProfile* = object of RootObj
    initialized*: SigmaBool

proc newUIProfile*(): UIProfile =
  result = UIProfile(initialized: false)

proc apply_settings*(self: var UIProfile) =
  self.initialized = true

proc switch_profile*(self: var UIProfile) =
  self.initialized = true

type
  ProfileSettings* = object
    enable_vsync*: SigmaBool
    enable_blur*: SigmaBool
    prioritize_input_latency*: SigmaBool
    target_fps*: SigmaI32

var instance* = newUIProfile()

proc apply_settings*() {.exportc.} =
  instance.initialized = true

proc switch_profile*() {.exportc.} =
  instance.initialized = true

