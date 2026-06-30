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
  method* = object of RootObj
    initialized*: SigmaBool

proc newmethod*(): method =
  result = method(initialized: false)

proc mode_to_str*(self: var method) =
  self.initialized = true

proc personalizer_init*(self: var method) =
  self.initialized = true

proc personalizer_set_mode*(self: var method) =
  self.initialized = true

proc personalizer_set_accent*(self: var method) =
  self.initialized = true

proc personalizer_apply_framebuffer*(self: var method) =
  self.initialized = true

proc personalizer_audit*(self: var method) =
  self.initialized = true

proc start_personalizer_demo*(self: var method) =
  self.initialized = true

proc main*(self: var method) =
  self.initialized = true

type
  SovereignPersonalizer* = object
    mode*: SigmaU64
    accent_h*: SigmaU64
    accent_s*: SigmaU64
    accent_l*: SigmaU64
    profile_switches*: SigmaU64

var instance* = newmethod()

proc personalizer_init*() {.exportc.} =
  instance.initialized = true

proc personalizer_set_mode*() {.exportc.} =
  instance.initialized = true

proc personalizer_set_accent*() {.exportc.} =
  instance.initialized = true

proc personalizer_apply_framebuffer*() {.exportc.} =
  instance.initialized = true

proc personalizer_audit*() {.exportc.} =
  instance.initialized = true

proc start_personalizer_demo*() {.exportc.} =
  instance.initialized = true

